import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button } from "primereact/button";
import { Calendar } from "primereact/calendar";
import { Dropdown } from 'primereact/dropdown';
import { InputNumber } from 'primereact/inputnumber';
import { useNotification } from '../common/NotificationProvider';
import { Card } from "primereact/card";
import { Fiat } from './fiatramp.model';

// Removed local Fiat interface

interface FundingCreateFormProps {
    onCancel?: () => void;
    onCreate?: () => void;
}

export default function FundingCreateForm({ onCancel, onCreate }: FundingCreateFormProps) {
    const { showSuccess, showError } = useNotification();
    const [fiat, setFiat] = useState('');
    const [fiatAmount, setFiatAmount] = useState<number>(0.00);
    const [fiats, setFiats] = useState<Fiat[]>([]);
    const [rampDate, setRampDate] = useState('');
    const [viaExchange, setViaExchange] = useState('');
    const [kind, setKind] = useState('deposit');

    const loadAllFiats = async () => {
        const fiats = await invoke<Fiat[]>('get_all_fiat');
        setFiats(fiats);
    }

    useEffect(() => {
        loadAllFiats();
    }, []);

    async function handleSubmit(e: React.FormEvent<HTMLFormElement>) {
        e.preventDefault();
        await invoke('create_fiat_ramp', {
            createFiatRamp: {
                fiat_id: parseInt(fiat),
                fiat_amount: fiatAmount,
                ramp_date: rampDate,
                via_exchange: viaExchange,
                kind: kind,
            }
        }).then(() => {
            showSuccess('Funding created successfully');
            if (onCreate) {
                onCreate();
            }
        }).catch((error) => {
            showError(`Failed to create funding: ${error}`);
        });
    }


    // TODO: fiat , fiat amount , date , via exchange
    return (
        <Card>
            <form onSubmit={handleSubmit} className="flex flex-col gap-4 p-4 max-w-lg mx-auto bg-base-100 rounded-box">
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
                        <span className="label-text">Funding Date</span>
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

                <Button type="submit" label="Submit" icon="pi pi-check" />
                <Button type="button" label="Cancel" icon="pi pi-times" onClick={onCancel} />
            </form>
        </Card>
    );
}