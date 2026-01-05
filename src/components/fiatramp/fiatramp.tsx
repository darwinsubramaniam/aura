import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Button } from "primereact/button";
import { Calendar } from "primereact/calendar";
import { Dropdown } from 'primereact/dropdown';
import { InputNumber } from 'primereact/inputnumber';


interface Fiat {
    id: number;
    symbol: string;
    name: string;
}

export default function FiatRampForm() {
    const [fiat, setFiat] = useState('');
    const [fiatAmount, setFiatAmount] = useState<number>(0.00);
    const [fiats, setFiats] = useState<Fiat[]>([]);
    const [date, setDate] = useState('');
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
        console.log(`fiat: ${fiat}, fiatAmount: ${fiatAmount}, date: ${date}, viaExchange: ${viaExchange}`)
        await invoke('create_fiat_ramp', {
            createFiatRamp: {
                fiat_id: parseInt(fiat),
                fiat_amount: fiatAmount,
                date: date,
                via_exchange: viaExchange,
                kind: kind,
            }
        }).then(() => {
            alert('Fiat ramp created successfully');
        }).catch((error) => {
            alert(error);
        });
    }


    // TODO: fiat , fiat amount , date , via exchange
    return (
        <form onSubmit={handleSubmit} className="flex flex-col gap-4 p-4 max-w-lg mx-auto bg-base-100 shadow-xl rounded-box">
            <h2 className="text-2xl font-bold mb-4 text-center">Create Fiat Ramp {kind.charAt(0).toUpperCase() + kind.slice(1)} </h2>

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
                    <span className="label-text">Date</span>
                </label>
                <Calendar
                    value={date ? new Date(date) : null}
                    onChange={(e) => {
                        if (e.value) {
                            const dateObj = new Date(e.value);
                            const dateStr = dateObj.toISOString().split('T')[0];
                            setDate(dateStr);
                        } else {
                            setDate('');
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
        </form>
    );
}