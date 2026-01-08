import { Dialog } from "primereact/dialog";
import FiatRampTable from "./fiatramp-table";
import { useState } from "react";
import FiatRampCreateForm from "./fiatramp-create";
import { Button } from "primereact/button";
import { Toolbar } from "primereact/toolbar"
import FiatRampSummary from "./fiatramp-summary";

export default function FiatRamp() {
    const [createDialogVisible, setCreateDialogVisible] = useState(false);
    const [refreshTrigger, setRefreshTrigger] = useState(0);

    const onCreated = () => {
        setRefreshTrigger(prev => prev + 1);
        setCreateDialogVisible(false);
    };
    const onCancelCreate = () => {
        setCreateDialogVisible(false);
    };

    const startToolbarTemplate = () => {
        return (
            <div className="flex justify-content-end">
                <Button
                    onClick={() => setCreateDialogVisible(true)}
                    icon="pi pi-plus"
                    label="Add Fiat Ramp"
                    raised
                />
            </div>
        );
    };


    return (
        <div>
            <Toolbar className="mb-4" start={startToolbarTemplate} />
            <div className="mb-4">
                <FiatRampSummary />
            </div>
            <FiatRampTable refreshTrigger={refreshTrigger} />
            <Dialog header="Create Fiat Ramp" visible={createDialogVisible} onHide={onCancelCreate}>
                <FiatRampCreateForm onCancel={onCancelCreate} onCreate={onCreated} />
            </Dialog>

        </div>
    );
}