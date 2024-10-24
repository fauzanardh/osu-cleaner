import { Video, Image, ScrollText, FileMusic, Images } from 'lucide-svelte';

interface CounterUpdate {
    background_video?: number;
    background_image?: number;
    storyboard?: number;
    skin_element?: number;
    hitsound?: number;
    other?: number;
}

interface CategoryDetailSimple {
    total_size: number;
    total_count: number;
}

interface CategorySummaryResponse {
    background_video: CategoryDetailSimple;
    background_image: CategoryDetailSimple;
    storyboard: CategoryDetailSimple;
    skin_element: CategoryDetailSimple;
    hitsound: CategoryDetailSimple;
}

interface FileInfo {
    path: string;
    size: number;
}

interface AlertType {
    show: boolean;
    message: string;
    title: string;
}

interface FilterCounts {
    background_videos: number;
    background_images: number;
    storyboards: number;
    hitsounds: number;
    skin_elements: number;
    other: number;
}

interface AnalyzerState {
    status: 'idle' | 'scanning' | 'parsing' | 'filtering' | 'complete';
    summary: CategorySummaryResponse | null;
    counts: {
        scanned: number;
        parsed: number;
        filtered: FilterCounts;
    };
    ui: {
        analyzer: {
            title: string;
            subtitle: string;
        };
    }
}

interface AlertMessage {
    id: string;
    type: 'success' | 'error' | 'warning' | 'info';
    title: string;
    message: string;
    persist?: boolean;
    timeout?: number;
    details?: unknown;
}

interface CategoryUI {
    id: string;
    icon: typeof Video | typeof Image | typeof ScrollText | typeof FileMusic | typeof Images;
    title: string;
    description: string;
    selected: boolean;
    size: number;
    count: number;
}

interface CategoryState {
    [key: string]: CategoryUI;
}

interface CategoryDataResponse {
    files: FileInfo[];
}

export type {
    CounterUpdate,
    CategorySummaryResponse,
    CategoryDataResponse,
    FileInfo,
    AlertType,
    FilterCounts,
    AnalyzerState,
    CategoryState,
    CategoryUI,
    AlertMessage
}