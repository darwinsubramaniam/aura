import { useEffect, useState } from "react";
import { useNotification } from '../common/NotificationProvider';

import { Button } from "@/components/ui/button";
import { Calendar } from "@/components/ui/calendar";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
    Select,
    SelectContent,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";
import {
    Popover,
    PopoverContent,
    PopoverTrigger,
} from "@/components/ui/popover"
import { cn } from "@/lib/utils";
import { CalendarIcon, Check, X } from "lucide-react";
import { format } from "date-fns";
import { FiatCommand } from "@/lib/services/fiat/fiat.command";
import { FiatRamp, UpdateFiatRamp } from "@/lib/models/fiatRamp";
import { Fiat } from "@/lib/models/fiat";
import { FiatRampCommand } from "@/lib/services/funding/fiatRamp.command";

interface FundingEditFormProps {
    fiatRamp: FiatRamp;
    onUpdated?: () => void;
    onCancel?: () => void;
}

export default function FundingEditForm({ fiatRamp, onUpdated, onCancel }: FundingEditFormProps) {
    const { showSuccess, showError } = useNotification();
    // Initialize state with props
    const [fiat, setFiat] = useState<string>(fiatRamp.fiat_id.toString());
    const [fiatAmount, setFiatAmount] = useState<number>(fiatRamp.fiat_amount);
    const [fiats, setFiats] = useState<Fiat[]>([]);
    const [rampDate, setRampDate] = useState<Date | undefined>(fiatRamp.ramp_date ? new Date(fiatRamp.ramp_date) : undefined);
    const [viaExchange, setViaExchange] = useState(fiatRamp.via_exchange);
    const [kind, setKind] = useState(fiatRamp.kind);

    const loadAllFiats = async () => {
        const fiats = await FiatCommand.getAllCurrencies();
        setFiats(fiats);
    }

    useEffect(() => {
        loadAllFiats();
    }, []);

    // Update state if prop changes
    useEffect(() => {
        setFiat(fiatRamp.fiat_id.toString());
        setFiatAmount(fiatRamp.fiat_amount);
        setRampDate(fiatRamp.ramp_date ? new Date(fiatRamp.ramp_date) : undefined);
        setViaExchange(fiatRamp.via_exchange);
        setKind(fiatRamp.kind);
    }, [fiatRamp]);

    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();

        // Prepare object for backend
        const updatedRamp: UpdateFiatRamp = {
            id: fiatRamp.id,
            fiat_id: parseInt(fiat),
            fiat_amount: fiatAmount,
            ramp_date: rampDate,
            via_exchange: viaExchange,
            kind: kind,
        };

        await FiatRampCommand.update(updatedRamp).then(() => {
            showSuccess('Funding updated successfully');
            if (onUpdated) {
                onUpdated();
            }
        }).catch((error) => {
            showError(`Failed to update funding: ${error}`);
        });
    }

    return (
        <form onSubmit={handleSubmit} className="flex flex-col gap-4">
            <div className="grid w-full items-center gap-1.5">
                <Label htmlFor="kind">Kind</Label>
                <Select value={kind} onValueChange={(v) => setKind(v as "deposit" | "withdraw")}>
                    <SelectTrigger id="kind" className="shadow-sm">
                        <SelectValue placeholder="Select kind" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="deposit">Deposit</SelectItem>
                        <SelectItem value="withdraw">Withdraw</SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div className="grid w-full items-center gap-1.5">
                <Label htmlFor="fiat">Fiat</Label>
                <Select value={fiat} onValueChange={setFiat}>
                    <SelectTrigger id="fiat" className="shadow-sm">
                        <SelectValue placeholder="Select Fiat" />
                    </SelectTrigger>
                    <SelectContent>
                        {fiats.map((f) => (
                            <SelectItem key={f.id} value={f.id.toString()}>
                                {f.name}
                            </SelectItem>
                        ))}
                    </SelectContent>
                </Select>
            </div>

            <div className="grid w-full items-center gap-1.5">
                <Label htmlFor="fiatAmount">Fiat Amount</Label>
                <Input
                    type="number"
                    id="fiatAmount"
                    step="0.01"
                    value={fiatAmount}
                    onChange={(e) => setFiatAmount(parseFloat(e.target.value))}
                    className="shadow-sm"
                />
            </div>

            <div className="grid w-full items-center gap-1.5">
                <Label>On (Date)</Label>
                <Popover>
                    <PopoverTrigger asChild>
                        <Button
                            variant={"outline"}
                            className={cn(
                                "w-full justify-start text-left font-normal shadow-sm",
                                !rampDate && "text-muted-foreground"
                            )}
                        >
                            <CalendarIcon className="mr-2 h-4 w-4" />
                            {rampDate ? format(rampDate, "PPP") : <span>Pick a date</span>}
                        </Button>
                    </PopoverTrigger>
                    <PopoverContent className="w-auto p-0">
                        <Calendar
                            mode="single"
                            selected={rampDate}
                            onSelect={setRampDate}
                            initialFocus
                        />
                    </PopoverContent>
                </Popover>
            </div>

            <div className="grid w-full items-center gap-1.5">
                <Label htmlFor="viaExchange">Via Exchange</Label>
                <Select value={viaExchange} onValueChange={setViaExchange}>
                    <SelectTrigger id="viaExchange" className="shadow-sm">
                        <SelectValue placeholder="Select Exchange" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="binance">Binance</SelectItem>
                        <SelectItem value="coinbase">Coinbase</SelectItem>
                        <SelectItem value="ftx">FTX</SelectItem>
                        <SelectItem value="kraken">Kraken</SelectItem>
                        <SelectItem value="uphold">Uphold</SelectItem>
                    </SelectContent>
                </Select>
            </div>

            <div className="flex justify-end gap-2 mt-4">
                <Button type="button" variant="secondary" onClick={onCancel}>
                    <X className="mr-2 h-4 w-4" /> Cancel
                </Button>
                <Button type="submit">
                    <Check className="mr-2 h-4 w-4" /> Update
                </Button>
            </div>
        </form>
    );

}
