<script lang="ts">
	import DailyStudyChart from '$lib/components/DailyStudyChart.svelte';
	import WeeklyStudyChart from '$lib/components/WeeklyStudyChart.svelte';
	import BookStatsChart from '$lib/components/BookStatsChart.svelte';

	const { data } = $props();
</script>

<div class="container mx-auto p-8">
	<h1 class="mb-8 text-3xl font-bold">Faith Statistics</h1>

	<div class="mb-8">
		<h2 class="mb-4 text-2xl font-semibold">Daily Study Time (Last 30 Days)</h2>
		<DailyStudyChart data={data.dailyStats} />
	</div>

	<div class="mb-8">
		<h2 class="mb-4 text-2xl font-semibold">Weekly Study Time (Last 12 Weeks)</h2>
		<WeeklyStudyChart data={data.weeklyStats} />
	</div>

	<div class="mb-8">
		<h2 class="mb-4 text-2xl font-semibold">Bible Books Progress</h2>
		<BookStatsChart data={data.bibleStats} />
	</div>
</div>
