import { DataTable, DataTablePageEvent } from 'primereact/datatable';
import { Column } from 'primereact/column';
import { InputText } from 'primereact/inputtext';
import { Button } from 'primereact/button';
import { ConfirmDialog, confirmDialog } from 'primereact/confirmdialog';
import { Toast } from 'primereact/toast';
import { useEffect, useState, useCallback, useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { IconField } from 'primereact/iconfield';
import { InputIcon } from 'primereact/inputicon';
import { Card } from 'primereact/card';
import { Dialog } from 'primereact/dialog';
import FiatRampEditForm from './fiatramp-edit';
import { FiatRamp, FiatRampPagination } from './fiatramp.model';

interface FiatRampTableProps {
    refreshTrigger?: number;
}

export default function FiatRampTable({ refreshTrigger }: FiatRampTableProps) {
    const [offset, setOffset] = useState(0);
    const [limit, setLimit] = useState(5);
    const [fiatRamps, setFiatRamps] = useState<FiatRamp[]>([]);
    const [selectedRamps, setSelectedRamps] = useState<FiatRamp[]>([]);
    const [totalRecords, setTotalRecords] = useState(0);
    const [loading, setLoading] = useState(true);
    const [globalFilter, setGlobalFilter] = useState<string>('');
    const [editDialogVisible, setEditDialogVisible] = useState(false);
    const [editingRamp, setEditingRamp] = useState<FiatRamp | null>(null);
    const toast = useRef<Toast>(null);

    const loadData = useCallback(() => {
        setLoading(true);
        invoke<FiatRampPagination>('get_all_fiat_ramps', { limit: limit, offset: offset, query: globalFilter }).then((res) => {
            setFiatRamps(res.fiat_ramps);
            setTotalRecords(res.total_count);
            setLoading(false);
        });
    }, [limit, offset, globalFilter]);

    useEffect(() => {
        loadData();
    }, [loadData, refreshTrigger]);

    const onPage = (event: DataTablePageEvent) => {
        setOffset(event.first);
        setLimit(event.rows);
    };

    const deleteFiatRamp = (id: string) => {
        invoke('delete_fiat_ramp', { id }).then(() => {
            loadData();
        });
    };

    const deleteSelectedRamps = () => {
        if (selectedRamps.length === 0) return;

        const promises = selectedRamps.map((ramp) => invoke('delete_fiat_ramp', { id: ramp.id }));
        Promise.all(promises).then(() => {
            loadData();
            setSelectedRamps([]);
            toast.current?.show({ severity: 'success', summary: 'Successful', detail: 'Products Deleted', life: 3000 });
        });
    };

    const confirmDeleteSelected = () => {
        confirmDialog({
            message: 'Are you sure you want to delete the selected products?',
            header: 'Confirmation',
            icon: 'pi pi-exclamation-triangle',
            accept: deleteSelectedRamps
        });
    };

    const confirmDeleteFiatRamp = (fiatRamp: FiatRamp) => {
        confirmDialog({
            message: 'Are you sure you want to delete ' + fiatRamp.id + '?',
            header: 'Confirmation',
            icon: 'pi pi-exclamation-triangle',
            accept: () => deleteFiatRamp(fiatRamp.id)
        });
    };

    const openEditDialog = (fiatRamp: FiatRamp) => {
        setEditingRamp(fiatRamp);
        setEditDialogVisible(true);
    };

    const hideEditDialog = () => {
        setEditDialogVisible(false);
        setEditingRamp(null);
    };

    const onRampUpdated = () => {
        hideEditDialog();
        loadData();
        toast.current?.show({ severity: 'success', summary: 'Successful', detail: 'Fiat Ramp Updated', life: 3000 });
    };

    const actionBodyTemplate = (rowData: FiatRamp) => {
        return (
            <div className="flex gap-2 justify-content-center">
                <Button icon="pi pi-pencil" rounded outlined className="mr-2" onClick={() => openEditDialog(rowData)} />
                <Button icon="pi pi-trash" rounded outlined severity="danger" onClick={() => confirmDeleteFiatRamp(rowData)} />
            </div>
        );
    };

    const header = (
        <div className="flex flex-wrap gap-2 align-items-center justify-content-between">
            <IconField iconPosition='left'>
                <InputIcon className="pi pi-search" />
                <InputText value={globalFilter} onChange={(e) => setGlobalFilter(e.target.value)} placeholder="Keyword Search" />
            </IconField>
            {selectedRamps.length > 0 && <Button label="Delete" icon="pi pi-trash" severity="danger" onClick={confirmDeleteSelected} />}
        </div>
    );

    return (
        <Card title="Manage Fiat Ramps">
            <Toast ref={toast} />
            <ConfirmDialog />
            <DataTable value={fiatRamps}
                selectionMode="multiple"
                lazy
                paginator
                first={offset}
                rows={limit}
                totalRecords={totalRecords}
                onPage={onPage}
                selection={selectedRamps}
                onSelectionChange={(e) => {
                    if (Array.isArray(e.value)) {
                        setSelectedRamps(e.value as FiatRamp[]);
                    }
                }}
                dataKey="id"
                rowsPerPageOptions={[5, 10, 20]}
                tableStyle={{ minWidth: '50rem' }}
                emptyMessage="No fiat ramps found"
                loading={loading}
                loadingIcon="pi pi-spinner pi-spin"
                header={header}
            >
                <Column selectionMode="multiple" headerStyle={{ width: '3rem' }}></Column>
                <Column field="fiat_symbol" sortable header="Fiat" />
                <Column field="fiat_amount" sortable header="Fiat Amount" />
                <Column field="date" sortable header="Date" />
                <Column field="via_exchange" sortable header="Via Exchange" />
                <Column field="kind" sortable header="Kind" />
                <Column body={actionBodyTemplate} header="Action" style={{ textAlign: 'center' }} />
            </DataTable>

            <Dialog visible={editDialogVisible} style={{ width: '450px' }} header="Edit Fiat Ramp" modal className="p-fluid" onHide={hideEditDialog}>
                {editingRamp && (
                    <FiatRampEditForm
                        fiatRamp={editingRamp}
                        onRampUpdated={onRampUpdated}
                        onCancel={hideEditDialog}
                    />
                )}
            </Dialog>
        </Card>
    );
}