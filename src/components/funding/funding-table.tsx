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
import FundingEditForm from './funding-edit';
import { Funding, FundingPagination } from './fiatramp.model';

interface FundingTableProps {
    refreshTrigger?: number;
}

export default function FundingTable({ refreshTrigger }: FundingTableProps) {
    const [offset, setOffset] = useState(0);
    const [limit, setLimit] = useState(5);
    const [funding, setFunding] = useState<Funding[]>([]);
    const [selectedFunding, setSelectedFunding] = useState<Funding[]>([]);
    const [totalRecords, setTotalRecords] = useState(0);
    const [loading, setLoading] = useState(true);
    const [globalFilter, setGlobalFilter] = useState<string>('');
    const [editDialogVisible, setEditDialogVisible] = useState(false);
    const [editingFunding, setEditingFunding] = useState<Funding | null>(null);
    const toast = useRef<Toast>(null);

    const loadData = useCallback(() => {
        setLoading(true);
        invoke<FundingPagination>('get_all_fiat_ramps', { limit: limit, offset: offset, query: globalFilter }).then((res) => {
            setFunding(res.fiat_ramps);
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
        if (selectedFunding.length === 0) return;

        const promises = selectedFunding.map((ramp) => invoke('delete_fiat_ramp', { id: ramp.id }));
        Promise.all(promises).then(() => {
            loadData();
            setSelectedFunding([]);
            toast.current?.show({ severity: 'success', summary: 'Successful', detail: 'Products Deleted', life: 3000 });
        });
    };

    const confirmDeleteSelected = () => {
        confirmDialog({
            message: 'Are you sure you want to delete the selected funding?',
            header: 'Confirmation',
            icon: 'pi pi-exclamation-triangle',
            accept: deleteSelectedRamps
        });
    };

    const confirmDeleteFiatRamp = (fiatRamp: Funding) => {
        confirmDialog({
            message: 'Are you sure you want to delete this funding?',
            header: 'Confirmation',
            icon: 'pi pi-exclamation-triangle',
            accept: () => deleteFiatRamp(fiatRamp.id)
        });
    };

    const openEditDialog = (fiatRamp: Funding) => {
        console.log(`Editing funding ${fiatRamp}`);
        setEditingFunding(fiatRamp);
        setEditDialogVisible(true);
    };

    const hideEditDialog = () => {
        setEditDialogVisible(false);
        setEditingFunding(null);
    };


    const onRampUpdated = () => {
        hideEditDialog();
        loadData();
        toast.current?.show({ severity: 'success', summary: 'Successful', detail: 'Funding Updated', life: 3000 });
    };


    const actionBodyTemplate = (rowData: Funding) => {
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
            {selectedFunding.length > 0 && <Button label="Delete" icon="pi pi-trash" severity="danger" onClick={confirmDeleteSelected} />}
        </div>
    );


    return (
        <div >
            <Card title="Funding History">
                <Toast ref={toast} />
                <ConfirmDialog />
                <DataTable value={funding}
                    selectionMode="multiple"
                    lazy
                    paginator
                    first={offset}
                    rows={limit}
                    totalRecords={totalRecords}
                    onPage={onPage}
                    selection={selectedFunding}
                    onSelectionChange={(e) => {
                        if (Array.isArray(e.value)) {
                            setSelectedFunding(e.value as Funding[]);
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
                    <Column field="ramp_date" sortable header="On (Date)" />
                    <Column field="via_exchange" sortable header="Via Exchange" />
                    <Column field="kind" sortable header="Kind" />
                    <Column body={actionBodyTemplate} header="Action" style={{ textAlign: 'center' }} />
                </DataTable>

                <Dialog visible={editDialogVisible} style={{ width: '450px' }} header="Edit Funding" modal className="p-fluid" onHide={hideEditDialog}>
                    {editingFunding && (
                        <FundingEditForm
                            fiatRamp={editingFunding}
                            onUpdated={onRampUpdated}
                            onCancel={hideEditDialog}
                        />
                    )}
                </Dialog>
            </Card>
        </div>

    );
}