<script lang="ts">
	import { Chart } from 'svelte5-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		BarElement,
		CategoryScale,
		LinearScale,
		LineElement,
		PointElement,
		LineController
	} from 'chart.js';
	import { Temporal } from '@js-temporal/polyfill';
	import type { FaithDailyStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';

	// Register Chart.js components
	ChartJS.register(
		Title,
		Tooltip,
		Legend,
		BarElement,
		CategoryScale,
		LinearScale,
		LineElement,
		PointElement,
		LineController
	);

	interface Props {
		data: FaithDailyStats;
	}

	const { data }: Props = $props();

	// Format date to show only month/day to prevent label overlap
	const formatDate = (dateStr: string) => {
		const date = Temporal.PlainDate.from(dateStr);
		return `${date.month}/${date.day}`;
	};

	// Calculate max absolute value for symmetric y-axis (left axis - matured/lost)
	const maxAbsValue = $derived.by(() => {
		const maturedValues = data.days.map((day) => day.anki_matured_passages);
		const lostValues = data.days.map((day) => day.anki_lost_passages);
		const allValues = [...maturedValues, ...lostValues];
		const max = Math.max(...allValues);
		// Add 10% padding and round up to nearest integer
		return Math.ceil(max * 1.1);
	});

	// Calculate max absolute value for symmetric y1-axis (right axis - cumulative)
	const maxAbsCumulative = $derived.by(() => {
		const cumulativeValues = data.days.map((day) => day.anki_cumulative_passages);
		const max = Math.max(...cumulativeValues.map(Math.abs));
		// Add 10% padding and round up to nearest integer
		return Math.ceil(max * 1.1);
	});

	// Transform data for Chart.js format (bar chart + line chart)
	const chartData = $derived({
		labels: data.days.map((day) => formatDate(day.date)),
		datasets: [
			// Bar dataset: Matured passages (positive values, above axis)
			{
				label: 'Matured',
				data: data.days.map((day) => day.anki_matured_passages),
				backgroundColor: chartColors.bar.background.green,
				borderColor: chartColors.bar.border.green,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.green,
				stack: 'stack0',
				yAxisID: 'y',
				order: 2
			},
			// Bar dataset: Lost passages (negative values, below axis)
			{
				label: 'Lost',
				data: data.days.map((day) => -day.anki_lost_passages),
				backgroundColor: chartColors.bar.background.red,
				borderColor: chartColors.bar.border.red,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.red,
				stack: 'stack0',
				yAxisID: 'y',
				order: 2
			},
			// Line dataset: Cumulative passages (right axis)
			{
				label: 'Cumulative',
				type: 'line' as const,
				data: data.days.map((day) => day.anki_cumulative_passages),
				borderColor: chartColors.bar.background.blue,
				backgroundColor: chartColors.bar.background.blue,
				borderWidth: 2,
				pointRadius: 3,
				pointHoverRadius: 5,
				yAxisID: 'y1',
				order: 1
			}
		]
	});

	const options = $derived({
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				display: true,
				position: 'top' as const
			},
			tooltip: {
				callbacks: {
					label: (context: { dataset: { label?: string }; parsed: { y: number | null } }) => {
						const value = context.parsed.y ?? 0;
						const label = context.dataset.label || '';
						// For the "Lost" dataset, show positive numbers in tooltip
						const displayValue = label === 'Lost' ? Math.abs(value) : value;
						return `${label}: ${displayValue} passages`;
					}
				}
			}
		},
		scales: {
			x: {
				stacked: true,
				grid: {
					display: false
				},
				ticks: {
					maxRotation: 45,
					minRotation: 45,
					font: {
						size: 11
					}
				}
			},
			y: {
				stacked: true,
				position: 'left' as const,
				min: -maxAbsValue,
				max: maxAbsValue,
				grid: {
					color: chartColors.grid.gray
				},
				title: {
					display: true,
					text: 'Passages'
				},
				ticks: {
					precision: 0,
					callback: function (value: string | number) {
						// Show absolute values on axis
						return Math.abs(typeof value === 'number' ? value : parseFloat(value));
					}
				}
			},
			y1: {
				position: 'right' as const,
				min: -maxAbsCumulative,
				max: maxAbsCumulative,
				grid: {
					display: false
				},
				title: {
					display: true,
					text: 'Cumulative Passages'
				},
				ticks: {
					precision: 0,
					callback: function (value: string | number) {
						// Show absolute values on axis
						return Math.abs(typeof value === 'number' ? value : parseFloat(value));
					}
				}
			}
		}
	});
</script>

<div class="h-64 w-full md:h-80">
	<Chart type="bar" data={chartData} {options} />
</div>
