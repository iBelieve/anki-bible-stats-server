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
	import type { BibleStats, BookStats } from '$lib/api/client';
	import chartColors from '$lib/theme/chartColors';

	// Custom plugin to draw a vertical line between OT and NT
	const testamentDividerPlugin = {
		id: 'testamentDivider',
		afterDatasetsDraw: (chart: ChartJS, _args: unknown, options: { ntStartIndex?: number }) => {
			const ntIdx = options.ntStartIndex;
			if (ntIdx === undefined || ntIdx === -1 || ntIdx === 0) return;

			const { ctx, chartArea, scales } = chart;
			const xScale = scales.x;

			// Calculate position between last OT book and first NT book
			const lastOTPosition = xScale.getPixelForValue(ntIdx - 1);
			const firstNTPosition = xScale.getPixelForValue(ntIdx);
			const dividerX = (lastOTPosition + firstNTPosition) / 2;

			ctx.save();
			ctx.strokeStyle = chartColors.label.gray;
			ctx.lineWidth = 2;
			ctx.setLineDash([5, 5]);
			ctx.beginPath();
			ctx.moveTo(dividerX, chartArea.top);
			ctx.lineTo(dividerX, chartArea.bottom);
			ctx.stroke();
			ctx.restore();
		}
	};

	// Register Chart.js components
	ChartJS.register(
		Title,
		Tooltip,
		Legend,
		BarElement,
		CategoryScale,
		LinearScale,
		testamentDividerPlugin
	);

	interface Props {
		data: BibleStats;
	}

	const { data }: Props = $props();

	// Combine OT and NT books, marking each with its testament
	interface BookWithTestament extends BookStats {
		testament: 'OT' | 'NT';
	}

	const allBooks: BookWithTestament[] = [
		...data.old_testament.book_stats.map((book) => ({ ...book, testament: 'OT' as const })),
		...data.new_testament.book_stats.map((book) => ({ ...book, testament: 'NT' as const }))
	];

	// Filter out books with no verses memorized
	const booksWithVerses = allBooks.filter(
		(book) => book.mature_verses > 0 || book.young_verses > 0
	);

	// Find the index where NT books start (for the divider)
	const ntStartIndex = booksWithVerses.findIndex((book) => book.testament === 'NT');

	// Transform data for Chart.js format (stacked bar chart)
	const chartData = $derived({
		labels: booksWithVerses.map((book) => book.book),
		datasets: [
			// Bottom stack: Mature verses
			{
				label: 'Mature',
				data: booksWithVerses.map((book) => book.mature_verses),
				backgroundColor: booksWithVerses.map((book) =>
					book.testament === 'OT'
						? chartColors.book.oldTestament.mature
						: chartColors.book.newTestament.mature
				),
				borderColor: booksWithVerses.map((book) =>
					book.testament === 'OT'
						? chartColors.book.oldTestament.border
						: chartColors.book.newTestament.border
				),
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: booksWithVerses.map((book) =>
					book.testament === 'OT'
						? chartColors.book.oldTestament.hover
						: chartColors.book.newTestament.hover
				),
				stack: 'stack0'
			},
			// Top stack: Young verses (lighter shade)
			{
				label: 'Young',
				data: booksWithVerses.map((book) => book.young_verses),
				backgroundColor: booksWithVerses.map((book) =>
					book.testament === 'OT'
						? chartColors.book.oldTestament.young
						: chartColors.book.newTestament.young
				),
				borderColor: booksWithVerses.map((book) =>
					book.testament === 'OT'
						? chartColors.book.oldTestament.border
						: chartColors.book.newTestament.border
				),
				borderWidth: 1,
				borderRadius: 4,
				hoverBackgroundColor: booksWithVerses.map((book) =>
					book.testament === 'OT'
						? chartColors.book.oldTestament.hover
						: chartColors.book.newTestament.hover
				),
				stack: 'stack0'
			}
		]
	});

	const options = {
		responsive: true,
		maintainAspectRatio: false,
		plugins: {
			testamentDivider: {
				ntStartIndex
			},
			legend: {
				display: true,
				position: 'top' as const,
				labels: {
					generateLabels: () => [
						{
							text: 'Old Testament',
							fillStyle: chartColors.book.oldTestament.mature,
							strokeStyle: chartColors.book.oldTestament.border,
							lineWidth: 1
						},
						{
							text: 'New Testament',
							fillStyle: chartColors.book.newTestament.mature,
							strokeStyle: chartColors.book.newTestament.border,
							lineWidth: 1
						}
					]
				}
			},
			tooltip: {
				callbacks: {
					label: (context: { dataset: { label?: string }; parsed: { y: number | null } }) => {
						const value = context.parsed.y ?? 0;
						const label = context.dataset.label || '';
						return `${label}: ${value} verses`;
					},
					footer: (tooltipItems: Array<{ parsed: { y: number | null } }>) => {
						const total = tooltipItems.reduce((sum, item) => sum + (item.parsed.y ?? 0), 0);
						return `Total: ${total} verses`;
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
					maxRotation: 90,
					minRotation: 45,
					font: {
						size: 10
					}
				}
			},
			y: {
				stacked: true,
				beginAtZero: true,
				grid: {
					color: chartColors.grid.gray
				},
				title: {
					display: true,
					text: 'Verses'
				},
				ticks: {
					precision: 0
				}
			}
		}
	};

	// Calculate statistics
	const otYoung = data.old_testament.young_verses;
	const otMature = data.old_testament.mature_verses;
	const otTotal = otYoung + otMature;

	const ntYoung = data.new_testament.young_verses;
	const ntMature = data.new_testament.mature_verses;
	const ntTotal = ntYoung + ntMature;

	const totalYoung = otYoung + ntYoung;
	const totalMature = otMature + ntMature;
	const grandTotal = totalYoung + totalMature;
</script>

<div class="h-64 w-full md:h-96">
	<Bar data={chartData} {options} />
</div>

<div class="mt-6 flex justify-center">
	<table>
		<thead>
			<tr>
				<th class="px-4 text-left"></th>
				<th class="px-4 text-right">Young</th>
				<th class="px-4 text-right">Mature</th>
				<th class="px-4 text-right">Total</th>
			</tr>
		</thead>
		<tbody>
			<tr>
				<td class="px-4 py-1">Old Testament</td>
				<td class="px-4 py-1 text-right">{otYoung}</td>
				<td class="px-4 py-1 text-right">{otMature}</td>
				<td class="px-4 py-1 text-right">{otTotal}</td>
			</tr>
			<tr>
				<td class="px-4 py-1">New Testament</td>
				<td class="px-4 py-1 text-right">{ntYoung}</td>
				<td class="px-4 py-1 text-right">{ntMature}</td>
				<td class="px-4 py-1 text-right">{ntTotal}</td>
			</tr>
			<tr class="border-t border-gray-300">
				<td class="px-4 py-1">Total</td>
				<td class="px-4 py-1 text-right">{totalYoung}</td>
				<td class="px-4 py-1 text-right">{totalMature}</td>
				<td class="px-4 py-1 text-right">{grandTotal}</td>
			</tr>
		</tbody>
	</table>
</div>
