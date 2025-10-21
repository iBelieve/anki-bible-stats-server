import colors from 'tailwindcss/colors';

/**
 * Chart color palette - organized by element type
 * Imports from Tailwind CSS for consistency
 * Single source of truth for all chart colors
 */
const chartColors = {
	bar: {
		background: {
			blue: colors.blue[500],
			red: colors.red[500],
			green: colors.green[500]
		},
		border: {
			blue: colors.blue[600],
			red: colors.red[600],
			green: colors.green[600]
		},
		hover: {
			blue: colors.blue[600],
			red: colors.red[600],
			green: colors.green[600]
		}
	},
	label: {
		red: colors.red[600],
		gray: colors.gray[500]
	},
	grid: {
		gray: colors.gray[200]
	}
};

export default chartColors;
