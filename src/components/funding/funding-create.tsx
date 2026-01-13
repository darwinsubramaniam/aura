import { useEffect, useState } from "react";
import { useNotification } from '../common/NotificationProvider';
import { Fiat } from '../../lib/models/fiat';
import { z } from 'zod';
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";
import { Input } from "@/components/ui/input";
import { Popover, PopoverContent, PopoverTrigger } from "@/components/ui/popover";
import { Calendar } from "@/components/ui/calendar";
import { cn } from "@/lib/utils";
import { CalendarIcon, Check, X } from "lucide-react";
import { format } from "date-fns";
import { FiatRampCommand } from "@/lib/services/funding/fiatRamp.command";
import { FiatCommand } from "@/lib/services/fiat/fiat.command";

interface FundingCreateFormProps {
    onCancel?: () => void;
    onCreate?: () => void;
}

const createFundingSchema = z.object({
    fiat: z.string().min(1, "Fiat currency is required"),
    fiatAmount: z.coerce.number().min(0.01, "Amount must be greater than 0"),
    rampDate: z.date(),
    viaExchange: z.string().min(1, "Exchange is required"),
    kind: z.enum(['deposit', 'withdraw']),
})

export default function FundingCreateForm({ onCancel, onCreate }: FundingCreateFormProps) {
    const { showSuccess, showError } = useNotification();
    const [fiats, setFiats] = useState<Fiat[]>([]);

    const form = useForm({
        resolver: zodResolver(createFundingSchema),
        defaultValues: {
            kind: 'deposit',
            fiatAmount: 0,
            fiat: "",
            viaExchange: "",
        }
    })

    const loadAllFiats = async () => {
        const fiats = await FiatCommand.getAllCurrencies();
        setFiats(fiats);
    }

    useEffect(() => {
        loadAllFiats();
    }, []);

    async function handleSubmit(value: z.infer<typeof createFundingSchema>) {
        try {
            await FiatRampCommand.create({
                fiat_id: parseInt(value.fiat),
                fiat_amount: value.fiatAmount,
                ramp_date: value.rampDate,
                via_exchange: value.viaExchange,
                kind: value.kind
            })
            showSuccess('Funding created successfully');
            if (onCreate) {
                onCreate();
            }
        }
        catch (error) {
            showError(`Failed to create funding: ${error}`);
        }
    }

    return (
        <Form {...form}>
            <form onSubmit={form.handleSubmit(handleSubmit)} className="space-y-4">
                <FormField
                    control={form.control}
                    name="kind"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Kind</FormLabel>
                            <Select onValueChange={field.onChange} defaultValue={field.value}>
                                <FormControl>
                                    <SelectTrigger className="shadow-sm">
                                        <SelectValue placeholder="Select kind" />
                                    </SelectTrigger>
                                </FormControl>
                                <SelectContent>
                                    <SelectItem value="deposit">Deposit</SelectItem>
                                    <SelectItem value="withdraw">Withdraw</SelectItem>
                                </SelectContent>
                            </Select>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="fiat"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Fiat</FormLabel>
                            <Select onValueChange={field.onChange} defaultValue={field.value}>
                                <FormControl>
                                    <SelectTrigger className="shadow-sm">
                                        <SelectValue placeholder="Select fiat" />
                                    </SelectTrigger>
                                </FormControl>
                                <SelectContent>
                                    {fiats.map((fiat) => (
                                        <SelectItem key={fiat.id} value={fiat.id.toString()}>
                                            {fiat.name}
                                        </SelectItem>
                                    ))}
                                </SelectContent>
                            </Select>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="fiatAmount"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Fiat Amount</FormLabel>
                            <FormControl>
                                <Input type="number" step="0.01" {...field} value={field.value as number} className="shadow-sm" />
                            </FormControl>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="rampDate"
                    render={({ field }) => (
                        <FormItem className="flex flex-col">
                            <FormLabel>On (Date)</FormLabel>
                            <Popover>
                                <PopoverTrigger asChild>
                                    <FormControl>
                                        <Button
                                            variant={"outline"}
                                            className={cn(
                                                "w-full pl-3 text-left font-normal shadow-sm",
                                                !field.value && "text-muted-foreground"
                                            )}
                                        >
                                            {field.value ? (
                                                format(field.value, "PPP")
                                            ) : (
                                                <span>Pick a date</span>
                                            )}
                                            <CalendarIcon className="ml-auto h-4 w-4 opacity-50" />
                                        </Button>
                                    </FormControl>
                                </PopoverTrigger>
                                <PopoverContent className="w-auto p-0" align="start">
                                    <Calendar
                                        mode="single"
                                        selected={field.value}
                                        onSelect={field.onChange}
                                        disabled={(date) => date > new Date()}
                                        initialFocus
                                    />
                                </PopoverContent>
                            </Popover>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <FormField
                    control={form.control}
                    name="viaExchange"
                    render={({ field }) => (
                        <FormItem>
                            <FormLabel>Via Exchange</FormLabel>
                            <Select onValueChange={field.onChange} defaultValue={field.value}>
                                <FormControl>
                                    <SelectTrigger className="shadow-sm">
                                        <SelectValue placeholder="Select Exchange" />
                                    </SelectTrigger>
                                </FormControl>
                                <SelectContent>
                                    <SelectItem value="binance">Binance</SelectItem>
                                    <SelectItem value="coinbase">Coinbase</SelectItem>
                                    <SelectItem value="ftx">FTX</SelectItem>
                                    <SelectItem value="kraken">Kraken</SelectItem>
                                    <SelectItem value="uphold">Uphold</SelectItem>
                                </SelectContent>
                            </Select>
                            <FormMessage />
                        </FormItem>
                    )}
                />

                <div className="flex justify-end gap-2">
                    <Button type="button" variant="secondary" onClick={onCancel}>
                        <X className="mr-2 h-4 w-4" /> Cancel
                    </Button>
                    <Button type="submit">
                        <Check className="mr-2 h-4 w-4" /> Create
                    </Button>
                </div>
            </form>
        </Form>
    );
}