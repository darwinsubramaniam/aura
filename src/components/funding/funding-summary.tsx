
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ReactECharts from 'echarts-for-react';
import { useEffect, useState } from "react";
import { Fiat } from "@/lib/models/fiat";
import { FiatCommand } from "@/lib/services/fiat/fiat.command";

export default function FundingSummary() {
    const [fiats, setFiats] = useState<Fiat[]>([]);

    const loadAllFiats = async () => {
        const fiats = await FiatCommand.getAllCurrencies();
        setFiats(fiats);
    }

    useEffect(() => {
        loadAllFiats();
    }, []);

    const option = {
        title: {
            text: 'Funding Summary',
            textStyle: {
                color: '#333'
            }
        },
        tooltip: {},
        legend: {
            data: ['Fiat Ramp']
        },
        xAxis: {
            type: 'category',
            data: fiats.map((f) => f.symbol.toUpperCase())
        },
        yAxis: {
            type: 'value'
        },
        series: [
            {
                name: 'Fiat Ramp',
                data: fiats.map((f) => 120), // logical placeholder
                type: 'bar'
            }
        ]
    };

    return (
        <Card>
            <CardHeader>
                <CardTitle>Funding Summary</CardTitle>
            </CardHeader>
            <CardContent>
                <ReactECharts option={option} />
            </CardContent>
        </Card>
    );
}