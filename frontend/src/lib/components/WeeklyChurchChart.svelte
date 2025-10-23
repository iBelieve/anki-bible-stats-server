<script lang="ts">
	import { Bar } from 'svelte5-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		BarElement,
		CategoryScale,
		LinearScale
	} from 'chart.js';
	import { Temporal } from '@js-temporal/polyfill';
	import type { FaithWeeklyStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';
	import { formatMinutesToHoursMinutes } from '$lib/utils/timeFormat';

	// Register Chart.js components
	ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale);

	interface Props {
		data: FaithWeeklyStats;
	}

	const { data }: Props = $props();

	// Format date to show month/day for week start
	const formatDate = (dateStr: string) => {
		const date = Temporal.PlainDate.from(dateStr);
		return `${date.month}/${date.day}`;
	};

	// Transform data for Chart.js format (single bar chart)
	const chartData = $derived({
		labels: data.weeks.map((week) => formatDate(week.week_start)),
		datasets: [
			{
				label: 'Church',
				data: data.weeks.map((week) => week.at_church_minutes),
				backgroundColor: chartColors.bar.background.orange,
				borderColor: chartColors.bar.border.orange,
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: chartColors.bar.hover.orange
			}
		]
	});

	const options = {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			legend: {
				display: false // Hide legend since there's only one dataset
			},
			tooltip: {
				callbacks: {
					label: (context: { parsed: { y: number | null } }) => {
						const minutes = context.parsed.y ?? 0;
						return `Church: ${formatMinutesToHoursMinutes(minutes)}`;
					}
				}
			}
		},
		scales: {
			x: {
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
				beginAtZero: true,
				grid: {
					color: chartColors.grid.gray
				},
				title: {
					display: true,
					text: 'Hours'
				},
				ticks: {
					stepSize: 60, // Force ticks at exact hour intervals
					callback: function (value: string | number) {
						// Convert the tick value (minutes) to hours and minutes format
						const minutes = typeof value === 'number' ? value : parseFloat(value);
						return formatMinutesToHoursMinutes(minutes);
					}
				}
			}
		}
	};
</script>

<div class="h-64 w-full md:h-80">
	<Bar data={chartData} {options} />
</div>
