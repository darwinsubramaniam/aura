import { useState } from "react";
import FundingTable from "./funding-table";
import FundingCreateForm from "./funding-create";
import FundingSummary from "./funding-summary";

import { Button } from "@/components/ui/button";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Plus } from "lucide-react";

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

    return (
        <div>
            <div className="mb-4 flex justify-end">
                <Button
                    onClick={() => setCreateDialogVisible(true)}
                >
                    <Plus className="mr-2 h-4 w-4" />
                    Deposit/Withdraw
                </Button>
            </div>
            <div className="mb-4">
                <FundingSummary />
            </div>
            <FundingTable refreshTrigger={refreshTrigger} />

            <Dialog open={createDialogVisible} onOpenChange={setCreateDialogVisible}>
                <DialogContent>
                    <DialogHeader>
                        <DialogTitle>Deposit or Withdraw Fund</DialogTitle>
                    </DialogHeader>
                    <FundingCreateForm onCancel={onCancelCreate} onCreate={onCreated} />
                </DialogContent>
            </Dialog>

        </div>
    );
}