import { invoke } from '@tauri-apps/api/core';

import type { CategorySummaryResponse, CategoryDataResponse } from '$lib/utils/interfaces.ts';

export class FileProcessorService {
    async cancelOperation(): Promise<void> {
        try {
            await invoke('cancel_operation');
        } catch (error) {
            console.log('Error while cancelling operation:', error);
            throw error;
        }
    }

    async scanDirectory(path: string): Promise<void> {
        try {
            await invoke('scan_directory', { path });
        } catch (error) {
            console.log('Error while scanning directory:', error);
            throw error;
        }
    }

    async getCategorySummary(): Promise<CategorySummaryResponse> {
        try {
            return await invoke('get_category_summary');
        } catch (error) {
            console.log('Error while getting category summary:', error);
            throw error;
        }
    }

    async getCategoryData(category: string): Promise<CategoryDataResponse> {
        try {
            return await invoke('get_category_data', { category });
        } catch (error) {
            console.log('Error while getting category data:', error);
            throw error;
        }
    }

    async deleteFiles(categories: string[]): Promise<void> {
        try {
            await invoke('delete_files', { categories });
        } catch (error) {
            console.log('Error while deleting files:', error);
            throw error;
        }
    }
}