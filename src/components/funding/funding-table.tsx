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
import { ArrowUpDown, Pencil, Trash } from "lucide-react"
import { toast } from "sonner";
import { FiatRamp } from '@/lib/models/fiatRamp';
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

    const [funding, setFunding] = useState<FiatRamp[]>([]);
    const [totalRecords, setTotalRecords] = useState(0);
    const [loading, setLoading] = useState(true);

    const [editDialogVisible, setEditDialogVisible] = useState(false);
    const [editingFunding, setEditingFunding] = useState<FiatRamp | null>(null);
    const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
    const [itemToDelete, setItemToDelete] = useState<FiatRamp | null>(null);
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

    const confirmDeleteFiatRamp = (fiatRamp: FiatRamp) => {
        setItemToDelete(fiatRamp);
        setDeleteDialogOpen(true);
    };

    const openEditDialog = (fiatRamp: FiatRamp) => {
        setEditingFunding(fiatRamp);
        setEditDialogVisible(true);
    };

    const onRampUpdated = () => {
        setEditDialogVisible(false);
        setEditingFunding(null);
        loadData();
        toast.success("Funding Updated");
    };


    const columns: ColumnDef<FiatRamp>[] = useMemo(
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
                accessorKey: "fiat_symbol",
                header: ({ column }) => {
                    return (
                        <Button
                            variant="ghost"
                            onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
                        >
                            Fiat
                            <ArrowUpDown className="ml-2 h-4 w-4" />
                        </Button>
                    )
                },
            },
            {
                accessorKey: "fiat_amount",
                header: "Fiat Amount",
                cell: ({ row }) => {
                    const amount = parseFloat(row.getValue("fiat_amount"))
                    const formatted = new Intl.NumberFormat("en-US", {
                        style: "currency",
                        currency: "USD",
                    }).format(amount)
                    return <div className="font-medium">{formatted}</div>
                },
            },
            {
                accessorKey: "ramp_date",
                header: "On (Date)",
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
        []
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
                        <AlertDialogAction onClick={() => itemToDelete && deleteFiatRamp(itemToDelete.id)}>Continue</AlertDialogAction>
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