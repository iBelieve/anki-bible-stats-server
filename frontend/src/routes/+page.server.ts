import { getDailyStats, getWeeklyStats } from '$lib/api/client';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
	const [dailyStats, weeklyStats] = await Promise.all([getDailyStats(), getWeeklyStats()]);

	return {
		dailyStats,
		weeklyStats
	};
};
