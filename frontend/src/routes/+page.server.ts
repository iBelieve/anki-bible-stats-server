import { getFaithDailyStats, getWeeklyStats } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const [dailyStats, weeklyStats] = await Promise.all([getFaithDailyStats(), getWeeklyStats()]);

	return {
		dailyStats,
		weeklyStats
	};
};
