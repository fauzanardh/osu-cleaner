import type { AnalyzerState, CategorySummaryResponse, CounterUpdate } from '$lib/utils/interfaces';

const initialState: AnalyzerState = {
    status: 'idle',
    summary: null,
    counts: {
        scanned: 0,
        parsed: 0,
        filtered: {
            background_videos: 0,
            background_images: 0,
            storyboards: 0,
            hitsounds: 0,
            skin_elements: 0,
            other: 0
        }
    },
    ui: {
        analyzer: {
            title: 'Initializing...',
            subtitle: ''
        }
    }
}

const createAnalyzerState = () => {
    let analyzer = $state<AnalyzerState>(initialState);

    const reset = () => {
        analyzer = initialState;
    };

    const setStatus = (status: AnalyzerState['status']) => {
        analyzer.status = status;
    };

    const updateScanCount = (count: number) => {
        analyzer.counts.scanned += count;
        analyzer.ui.analyzer.subtitle = `Found ${analyzer.counts.scanned.toLocaleString()} files`;
    };

    const updateParseCount = (count: number) => {
        analyzer.counts.parsed += count;
        analyzer.ui.analyzer.subtitle = `Parsed ${analyzer.counts.parsed.toLocaleString()} files`;
    };

    const updateFilterCounts = (counts: CounterUpdate) => {
        analyzer.counts.filtered.background_videos += counts.background_video || 0;
        analyzer.counts.filtered.background_images += counts.background_image || 0;
        analyzer.counts.filtered.storyboards += counts.storyboard || 0;
        analyzer.counts.filtered.hitsounds += counts.hitsound || 0;
        analyzer.counts.filtered.skin_elements += counts.skin_element || 0;
        analyzer.counts.filtered.other += counts.other || 0;
    };

    const updateUI = (title: string, subtitle: string) => {
        analyzer.ui.analyzer.title = title;
        analyzer.ui.analyzer.subtitle = subtitle;
    };

    const setSummary = (summary: CategorySummaryResponse) => {
        analyzer.summary = summary;
    };

    return {
        get analyzer() {
            return analyzer;
        },
        reset,
        setStatus,
        updateScanCount,
        updateParseCount,
        updateFilterCounts,
        updateUI,
        setSummary
    };
};

export default createAnalyzerState;