import { Dialog } from "primereact/dialog";
import FundingTable from "./funding-table";
import { useState } from "react";
import FundingCreateForm from "./funding-create";
import { Button } from "primereact/button";
import { Toolbar } from "primereact/toolbar"
import FundingSummary from "./funding-summary";

export default function Funding() {
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
                    label="Deposit/Withdraw"
                    raised
                />
            </div>
        );
    };


    return (
        <div>
            <Toolbar className="mb-4" start={startToolbarTemplate} />
            <div className="mb-4">
                <FundingSummary />
            </div>
            <FundingTable refreshTrigger={refreshTrigger} />
            <Dialog header="Deposit or Withdraw Fund" visible={createDialogVisible} onHide={onCancelCreate}>
                <FundingCreateForm onCancel={onCancelCreate} onCreate={onCreated} />
            </Dialog>

        </div>
    );
}