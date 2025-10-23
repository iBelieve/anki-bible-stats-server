/**
 * Converts minutes to a formatted string showing hours and minutes
 * Hours are always shown, minutes are optional (only shown if non-zero)
 * @param minutes - The number of minutes to format
 * @returns Formatted string like "2h 30m", "1h 15m", "0h 45m", or "2h"
 *
 * @example
 * formatMinutesToHoursMinutes(150) // "2h 30m"
 * formatMinutesToHoursMinutes(75)  // "1h 15m"
 * formatMinutesToHoursMinutes(45)  // "0h 45m"
 * formatMinutesToHoursMinutes(120) // "2h"
 * formatMinutesToHoursMinutes(0)   // "0h"
 */
export function formatMinutesToHoursMinutes(minutes: number): string {
	const hours = Math.floor(minutes / 60);
	const mins = Math.round(minutes % 60);

	if (mins === 0) {
		return `${hours}h`;
	}

	return `${hours}h ${mins}m`;
}
