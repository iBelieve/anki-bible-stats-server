import { getFaithDailyStats, getFaithWeeklyStats } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const [dailyStats, weeklyStats] = await Promise.all([
		getFaithDailyStats(),
		getFaithWeeklyStats()
	]);

	return {
		dailyStats,
		weeklyStats
	};
};
