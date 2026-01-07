import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button } from "primereact/button";
import { Calendar } from "primereact/calendar";
import { Dropdown } from 'primereact/dropdown';
import { InputNumber } from 'primereact/inputnumber';
import { useNotification } from '../common/NotificationProvider';
import { Card } from "primereact/card";
import { Fiat, FiatRamp } from './fiatramp.model';

interface FiatRampEditFormProps {
    fiatRamp: FiatRamp;
    onRampUpdated?: () => void;
    onCancel?: () => void;
}

export default function FiatRampEditForm({ fiatRamp, onRampUpdated, onCancel }: FiatRampEditFormProps) {
    const { showSuccess, showError } = useNotification();
    // Initialize state with props
    const [fiat, setFiat] = useState<any>(fiatRamp.fiat_id); // Dropdown value
    const [fiatAmount, setFiatAmount] = useState<number>(fiatRamp.fiat_amount);
    const [fiats, setFiats] = useState<Fiat[]>([]);
    const [rampDate, setRampDate] = useState(fiatRamp.ramp_date);
    const [viaExchange, setViaExchange] = useState(fiatRamp.via_exchange);
    const [kind, setKind] = useState(fiatRamp.kind);

    const loadAllFiats = async () => {
        const fiats = await invoke<Fiat[]>('get_all_fiat');
        setFiats(fiats);
    }

    useEffect(() => {
        loadAllFiats();
    }, []);

    // Update state if prop changes (e.g. user selects different row while dialog is open, though unlikely if modal)
    useEffect(() => {
        setFiat(fiatRamp.fiat_id);
        setFiatAmount(fiatRamp.fiat_amount);
        setRampDate(fiatRamp.ramp_date);
        setViaExchange(fiatRamp.via_exchange);
        setKind(fiatRamp.kind);
    }, [fiatRamp]);

    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();

        // Prepare object for backend
        const updatedRamp = {
            id: fiatRamp.id,
            fiat_id: fiat,
            fiat_amount: fiatAmount,
            date: rampDate,
            via_exchange: viaExchange,
            kind: kind,
            created_at: fiatRamp.created_at,
            updated_at: fiatRamp.updated_at
        };

        await invoke('update_fiat_ramp', {
            fiatRamp: updatedRamp
        }).then(() => {
            showSuccess('Fiat ramp updated successfully');
            if (onRampUpdated) {
                onRampUpdated();
            }
        }).catch((error) => {
            showError(`Failed to update fiat ramp: ${error}`);
        });
    }

    return (
        <Card title={`Edit Fiat Ramp`}>
            <form onSubmit={handleSubmit} className="flex flex-col gap-4 p-4 max-w-lg mx-auto bg-base-100 shadow-xl rounded-box">
                <div className="form-control w-full">
                    <label className="label" htmlFor="kind">
                        <span className="label-text">Kind</span>
                    </label>
                    <Dropdown
                        id="kind"
                        value={kind}
                        onChange={(e) => setKind(e.value)}
                        options={[
                            { name: 'Deposit', value: 'deposit' },
                            { name: 'Withdraw', value: 'withdraw' },
                        ]}
                        optionLabel="name"
                        optionValue="value"
                        className="w-full"
                    />
                </div>

                <div className="form-control w-full">
                    <label className="label" htmlFor="fiat">
                        <span className="label-text">Fiat</span>
                    </label>
                    <Dropdown
                        id="fiat"
                        value={fiat}
                        onChange={(e) => setFiat(e.value)}
                        options={fiats}
                        optionLabel="name"
                        optionValue="id"
                        className="w-full"
                        placeholder="Select Fiat"
                    />
                </div>

                <div className="form-control w-full">
                    <label className="label" htmlFor="fiatAmount">
                        <span className="label-text">Fiat Amount</span>
                    </label>
                    <InputNumber inputId="currency" value={fiatAmount} onValueChange={(e) => setFiatAmount(e.value ? e.value : 0.00)} mode="currency" currency="USD" className="w-full" />
                </div>

                <div className="form-control w-full">
                    <label className="label" htmlFor="date">
                        <span className="label-text">On (Date)</span>
                    </label>
                    <Calendar
                        value={rampDate ? new Date(rampDate) : null}
                        onChange={(e) => {
                            if (e.value) {
                                const dateObj = new Date(e.value);
                                const dateStr = dateObj.toISOString().split('T')[0];
                                setRampDate(dateStr);
                            } else {
                                setRampDate('');
                            }
                        }}
                        locale="en"
                        dateFormat="yy-mm-dd"
                        showIcon
                        className="w-full"
                    />
                </div>


                <div className="form-control w-full">
                    <label className="label" htmlFor="viaExchange">
                        <span className="label-text">Via Exchange</span>
                    </label>
                    <Dropdown
                        id="viaExchange"
                        value={viaExchange}
                        onChange={(e) => setViaExchange(e.value)}
                        options={[
                            { name: 'Binance', value: 'binance' },
                            { name: 'Coinbase', value: 'coinbase' },
                            { name: 'FTX', value: 'ftx' },
                            { name: 'Kraken', value: 'kraken' },
                            { name: 'Uphold', value: 'uphold' },
                        ]}
                        optionLabel="name"
                        optionValue="value"
                        className="w-full"
                    />
                </div>

                <div className="flex gap-2 justify-content-end">
                    <Button type="button" label="Cancel" icon="pi pi-times" severity="secondary" onClick={onCancel} />
                    <Button type="submit" label="Update" icon="pi pi-check" />
                </div>
            </form>
        </Card>
    );
}
