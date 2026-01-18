"use client"

import {
    ColumnDef,
    flexRender,
    getCoreRowModel,
    useReactTable,
    PaginationState,
    SortingState,
    RowSelectionState,
    VisibilityState,
} from "@tanstack/react-table"

import {
    ChevronLeft,
    ChevronRight,
    ChevronsLeft,
    ChevronsRight,
} from "lucide-react"

import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Skeleton } from "@/components/ui/skeleton"

interface DataTableProps<TData, TValue> {
    columns: ColumnDef<TData, TValue>[]
    data: TData[]
    rowCount?: number
    pagination?: PaginationState
    onPaginationChange?: (pagination: PaginationState) => void
    sorting?: SortingState
    onSortingChange?: (sorting: SortingState) => void
    rowSelection?: RowSelectionState
    onRowSelectionChange?: (rowSelection: RowSelectionState) => void
    columnVisibility?: VisibilityState
    onColumnVisibilityChange?: (columnVisibility: VisibilityState) => void
    loading?: boolean
    skeletonRowCount?: number
    searchKey?: string
    searchValue?: string
    onSearchChange?: (value: string) => void
    getRowId?: (row: TData) => string
    noResultsMessage?: React.ReactNode
}

export function DataTable<TData, TValue>({
    columns,
    data,
    rowCount,
    pagination,
    onPaginationChange,
    sorting,
    onSortingChange,
    rowSelection,
    onRowSelectionChange,
    columnVisibility,
    onColumnVisibilityChange,
    loading,
    skeletonRowCount = 5,
    searchKey,
    searchValue,
    onSearchChange,
    getRowId,
    noResultsMessage = "No results.",
}: DataTableProps<TData, TValue>) {

    // If we manage pagination server side, we rely on the props.
    // Otherwise we can default to local state if needed (not implemented here for simplicity as we target server side)

    const table = useReactTable({
        data,
        columns,
        rowCount,
        getCoreRowModel: getCoreRowModel(),
        manualPagination: true,
        onPaginationChange: (updater) => {
            if (onPaginationChange && pagination) {
                // Handle updater which can be a function or value
                const nextPagination = typeof updater === 'function' ? updater(pagination) : updater
                onPaginationChange(nextPagination)
            }
        },
        state: {
            pagination,
            sorting,
            rowSelection,
            columnVisibility,
        },
        manualSorting: true,
        onSortingChange: (updater) => {
            if (onSortingChange && sorting) {
                const nextSorting = typeof updater === 'function' ? updater(sorting) : updater
                onSortingChange(nextSorting)
            }
        },
        onRowSelectionChange: (updater) => {
            if (onRowSelectionChange && rowSelection) {
                const nextRowSelection = typeof updater === 'function' ? updater(rowSelection) : updater
                onRowSelectionChange(nextRowSelection)
            }
        },
        onColumnVisibilityChange: (updater) => {
            if (onColumnVisibilityChange && columnVisibility) {
                const nextColumnVisibility = typeof updater === 'function' ? updater(columnVisibility) : updater
                onColumnVisibilityChange(nextColumnVisibility)
            }
        },
        getRowId: getRowId || ((row: any) => row.id), // Use prop or default to 'id'
    })

    return (
        <div>
            {(searchKey || onSearchChange) && (
                <div className="flex items-center py-4">
                    {onSearchChange && (
                        <Input
                            placeholder="Search..."
                            value={searchValue ?? ""}
                            onChange={(event) => onSearchChange(event.target.value)}
                            className="max-w-sm"
                        />
                    )}
                </div>
            )}

            <div className="rounded-md border">
                <Table>
                    <TableHeader>
                        {table.getHeaderGroups().map((headerGroup) => (
                            <TableRow key={headerGroup.id}>
                                {headerGroup.headers.map((header) => {
                                    return (
                                        <TableHead key={header.id}>
                                            {header.isPlaceholder
                                                ? null
                                                : flexRender(
                                                    header.column.columnDef.header,
                                                    header.getContext()
                                                )}
                                        </TableHead>
                                    )
                                })}
                            </TableRow>
                        ))}
                    </TableHeader>
                    <TableBody>
                        {loading ? (
                            Array.from({ length: skeletonRowCount }).map((_, rowIndex) => (
                                <TableRow key={`skeleton-${rowIndex}`}>
                                    {columns.map((_, colIndex) => (
                                        <TableCell key={`skeleton-${rowIndex}-${colIndex}`}>
                                            <Skeleton className="h-6 w-full" />
                                        </TableCell>
                                    ))}
                                </TableRow>
                            ))
                        ) : table.getRowModel().rows?.length ? (
                            table.getRowModel().rows.map((row) => (
                                <TableRow
                                    key={row.id}
                                    data-state={row.getIsSelected() && "selected"}
                                >
                                    {row.getVisibleCells().map((cell) => (
                                        <TableCell key={cell.id}>
                                            {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                        </TableCell>
                                    ))}
                                </TableRow>
                            ))
                        ) : (
                            <TableRow>
                                <TableCell colSpan={columns.length} className="h-24 text-center">
                                    {noResultsMessage}
                                </TableCell>
                            </TableRow>
                        )}
                    </TableBody>
                </Table>
            </div>
            <div className="flex items-center justify-between px-2 py-4">
                <div className="flex-1 text-sm text-muted-foreground">
                    {table.getFilteredSelectedRowModel().rows.length} of{" "}
                    {rowCount ?? data.length} row(s) selected.
                </div>
                <div className="flex items-center space-x-6 lg:space-x-8">
                    <div className="flex w-[100px] items-center justify-center text-sm font-medium">
                        Page {table.getState().pagination.pageIndex + 1} of{" "}
                        {table.getPageCount()}
                    </div>
                    <div className="flex items-center space-x-2">
                        <Button
                            variant="outline"
                            className="h-8 w-8 p-0"
                            onClick={() => table.setPageIndex(0)}
                            disabled={!table.getCanPreviousPage()}
                        >
                            <span className="sr-only">Go to first page</span>
                            <ChevronsLeft className="h-4 w-4" />
                        </Button>
                        <Button
                            variant="outline"
                            className="h-8 w-8 p-0"
                            onClick={() => table.previousPage()}
                            disabled={!table.getCanPreviousPage()}
                        >
                            <span className="sr-only">Go to previous page</span>
                            <ChevronLeft className="h-4 w-4" />
                        </Button>
                        <Button
                            variant="outline"
                            className="h-8 w-8 p-0"
                            onClick={() => table.nextPage()}
                            disabled={!table.getCanNextPage()}
                        >
                            <span className="sr-only">Go to next page</span>
                            <ChevronRight className="h-4 w-4" />
                        </Button>
                        <Button
                            variant="outline"
                            className="h-8 w-8 p-0"
                            onClick={() => table.setPageIndex(table.getPageCount() - 1)}
                            disabled={!table.getCanNextPage()}
                        >
                            <span className="sr-only">Go to last page</span>
                            <ChevronsRight className="h-4 w-4" />
                        </Button>
                    </div>
                </div>
            </div>
        </div>
    )
}
