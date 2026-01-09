
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import ReactECharts from 'echarts-for-react';
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";
import { Fiat } from "./fiatramp.model";

export default function FundingSummary() {
    const [fiats, setFiats] = useState<Fiat[]>([]);

    const loadAllFiats = async () => {
        const fiats = await invoke<Fiat[]>('get_all_fiat');
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