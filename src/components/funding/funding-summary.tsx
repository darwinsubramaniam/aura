import { Card } from "primereact/card";
import * as echarts from 'echarts';
import { useEffect, useRef } from "react";

export default function FundingSummary() {
    const summaryChartRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (summaryChartRef.current) {
            const chart = echarts.init(summaryChartRef.current);
            chart.setOption({
                title: {
                    text: 'Funding Summary',
                    textStyle: {
                        color: '#ffffff'
                    }
                },
                tooltip: {},
                legend: {
                    data: ['Fiat Ramp']
                },
                xAxis: {
                    data: ['2022', '2023', '2024', '2025', '2026', '2027', '2028']
                },
                yAxis: {},
                series: [{
                    name: 'Fiat Ramp',
                    type: 'bar',
                    data: [120, 200, 150, 80, 70, 110, 130]
                }]
            });
        }
    }, []);

    return (
        <Card title="Funding Summary">
            <div ref={summaryChartRef} style={{ width: '100%', height: '400px' }}></div>
        </Card>
    );
}