import { useEffect, useState, useCallback, useMemo } from 'react';
import { invoke } from '@tauri-apps/api/core';
import FundingEditForm from './funding-edit';

import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import {
    AlertDialog,
    AlertDialogAction,
    AlertDialogCancel,
    AlertDialogContent,
    AlertDialogDescription,
    AlertDialogFooter,
    AlertDialogHeader,
    AlertDialogTitle,
} from "@/components/ui/alert-dialog"
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '@/components/ui/dialog';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';

import { ColumnDef, PaginationState, RowSelectionState, SortingState } from "@tanstack/react-table"
import { DataTable } from "@/components/ui/data-table"
import { Pencil, Trash, Clock } from "lucide-react"
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { toast } from "sonner";
import { FiatRampView } from '@/lib/models/fiatRamp';
import { FiatRampCommand } from '@/lib/services/funding/fiatRamp.command';


interface FundingTableProps {
    refreshTrigger?: number;
}

export default function FundingTable({ refreshTrigger }: FundingTableProps) {
    const [pagination, setPagination] = useState<PaginationState>({
        pageIndex: 0,
        pageSize: 5,
    })
    const [sorting, setSorting] = useState<SortingState>([])
    const [rowSelection, setRowSelection] = useState<RowSelectionState>({})
    const [globalFilter, setGlobalFilter] = useState<string>('');

    const [funding, setFunding] = useState<FiatRampView[]>([]);
    const [totalRecords, setTotalRecords] = useState(0);
    const [loading, setLoading] = useState(true);

    const [editDialogVisible, setEditDialogVisible] = useState(false);
    const [editingFunding, setEditingFunding] = useState<FiatRampView | null>(null);
    const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
    const [itemToDelete, setItemToDelete] = useState<FiatRampView | null>(null);
    const [deleteSelectedDialogOpen, setDeleteSelectedDialogOpen] = useState(false);

    const loadData = useCallback(() => {
        setLoading(true);
        const offset = pagination.pageIndex * pagination.pageSize;
        FiatRampCommand.get(pagination.pageSize, offset, globalFilter)
            .then((res) => {
                setFunding(res.fiat_ramps);
                setTotalRecords(res.total_count);
                setLoading(false);
            });
    }, [pagination.pageIndex, pagination.pageSize, globalFilter]);

    useEffect(() => {
        loadData();
    }, [loadData, refreshTrigger]);

    const deleteFiatRamp = (id: string) => {
        invoke('delete_fiat_ramp', { id }).then(() => {
            toast.success("Funding deleted successfully");
            setDeleteDialogOpen(false);
            setItemToDelete(null);
            loadData();
        });
    };

    const deleteSelectedRamps = () => {
        const selectedIds = Object.keys(rowSelection).filter(key => rowSelection[key]);
        if (selectedIds.length === 0) return;

        const promises = selectedIds.map((id) => invoke('delete_fiat_ramp', { id }));
        Promise.all(promises).then(() => {
            toast.success("Selected products deleted");
            setDeleteSelectedDialogOpen(false);
            setRowSelection({});
            loadData();
        });
    };

    const confirmDeleteFiatRamp = (fiatRamp: FiatRampView) => {
        setItemToDelete(fiatRamp);
        setDeleteDialogOpen(true);
    };

    const openEditDialog = (fiatRamp: FiatRampView) => {
        setEditingFunding(fiatRamp);
        setEditDialogVisible(true);
    };

    const onRampUpdated = () => {
        setEditDialogVisible(false);
        setEditingFunding(null);
        loadData();
        toast.success("Funding Updated");
    };


    const targetSymbol = funding.length > 0 ? funding[0].to_fiat_symbol : "";

    const columns: ColumnDef<FiatRampView>[] = useMemo(
        () => [
            {
                id: "select",
                header: ({ table }) => (
                    <Checkbox
                        checked={
                            table.getIsAllPageRowsSelected() ||
                            (table.getIsSomePageRowsSelected() && "indeterminate")
                        }
                        onCheckedChange={(value) => table.toggleAllPageRowsSelected(!!value)}
                        aria-label="Select all"
                    />
                ),
                cell: ({ row }) => (
                    <Checkbox
                        checked={row.getIsSelected()}
                        onCheckedChange={(value) => row.toggleSelected(!!value)}
                        aria-label="Select row"
                    />
                ),
                enableSorting: false,
                enableHiding: false,
            },
            {
                accessorKey: "ramp_date",
                header: "On (Date)",
                cell: ({ row }) => {
                    const date = row.getValue("ramp_date") as string;
                    const isEstimated = row.original.is_estimated;

                    return (
                        <div className="flex items-center gap-2">
                            <span>{date}</span>
                            {isEstimated && (
                                <Tooltip>
                                    <TooltipTrigger asChild>
                                        <Clock className="h-4 w-4 text-amber-500" />
                                    </TooltipTrigger>
                                    <TooltipContent>
                                        <p>Estimated rate (fallback from previous day)</p>
                                    </TooltipContent>
                                </Tooltip>
                            )}
                        </div>
                    );
                },
            },
            {
                accessorKey: "fiat_amount",
                header: "Fiat Amount",
                cell: ({ row }) => {
                    const amount = parseFloat(row.getValue("fiat_amount"))
                    const symbol = row.original.from_fiat_symbol || "USD";
                    const formatted = new Intl.NumberFormat("en-US", {
                        style: "currency",
                        currency: symbol,
                    }).format(amount)
                    return <div className="font-medium">{formatted}</div>
                },
            },
            {
                accessorKey: "converted_amount",
                header: targetSymbol ? `Converted (${targetSymbol})` : "Converted",
                cell: ({ row }) => {
                    const amount = row.original.converted_amount;
                    const symbol = row.original.to_fiat_symbol;
                    if (amount === null || amount === undefined) return <div className="text-muted-foreground">-</div>;
                    
                    const formatted = new Intl.NumberFormat("en-US", {
                        style: "currency",
                        currency: symbol || "USD", // Fallback, though view should provide it
                    }).format(amount)
                    return <div className="font-medium text-emerald-600">{formatted}</div>
                },
            },
            {
                accessorKey: "via_exchange",
                header: "Via Exchange",
            },
            {
                accessorKey: "kind",
                header: "Kind",
                cell: ({ row }) => {
                    return <div className="capitalize">{row.getValue("kind")}</div>
                },
            },
            {
                id: "actions",
                enableHiding: false,
                cell: ({ row }) => {
                    const payment = row.original

                    return (
                        <div className="flex gap-2">
                            <Button variant="outline" size="icon" onClick={() => openEditDialog(payment)}>
                                <Pencil className="h-4 w-4" />
                            </Button>
                            <Button variant="outline" size="icon" className="text-destructive hover:text-destructive" onClick={() => confirmDeleteFiatRamp(payment)}>
                                <Trash className="h-4 w-4" />
                            </Button>
                        </div>
                    )
                },
            },
        ],
        [targetSymbol]
    )

    return (
        <Card className='w-full'>
            <CardHeader className='flex flex-row items-center justify-between space-y-0 pb-2'>
                <CardTitle>Funding History</CardTitle>
                {Object.keys(rowSelection).length > 0 && (
                    <Button variant="destructive" size="sm" onClick={() => setDeleteSelectedDialogOpen(true)}>
                        Delete Selected
                    </Button>
                )}
            </CardHeader>
            <CardContent>
                <DataTable
                    columns={columns}
                    data={funding}
                    rowCount={totalRecords}
                    pagination={pagination}
                    onPaginationChange={setPagination}
                    sorting={sorting}
                    onSortingChange={setSorting}
                    rowSelection={rowSelection}
                    onRowSelectionChange={setRowSelection}
                    loading={loading}
                    searchValue={globalFilter}
                    onSearchChange={setGlobalFilter}
                    getRowId={(row) => row.fiat_ramp_id} 
                    searchKey="global" // We are doing global search
                />
            </CardContent>

            <AlertDialog open={deleteDialogOpen} onOpenChange={setDeleteDialogOpen}>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>Are you absolutely sure?</AlertDialogTitle>
                        <AlertDialogDescription>
                            This action cannot be undone. This will permanently delete the funding record.
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction onClick={() => itemToDelete && deleteFiatRamp(itemToDelete.fiat_ramp_id)}>Continue</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>

            <AlertDialog open={deleteSelectedDialogOpen} onOpenChange={setDeleteSelectedDialogOpen}>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>Delete all selected?</AlertDialogTitle>
                        <AlertDialogDescription>
                            This action cannot be undone. This will permanently delete {Object.keys(rowSelection).length} funding records.
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>Cancel</AlertDialogCancel>
                        <AlertDialogAction onClick={deleteSelectedRamps}>Continue</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>

            <Dialog open={editDialogVisible} onOpenChange={setEditDialogVisible}>
                <DialogContent className="sm:max-w-[425px]">
                    <DialogHeader>
                        <DialogTitle>Edit Funding</DialogTitle>
                    </DialogHeader>
                    {editingFunding && (
                        <FundingEditForm
                            fiatRamp={editingFunding}
                            onUpdated={onRampUpdated}
                            onCancel={() => setEditDialogVisible(false)}
                        />
                    )}
                </DialogContent>
            </Dialog>

        </Card>
    );
}