import { CategoryIconComponents } from "$lib/utils/icons";
import type { CategoryState, CategorySummaryResponse } from "$lib/utils/interfaces";


let initialState: CategoryState = {
    background_video: {
        id: 'background_video',
        icon: CategoryIconComponents['background_video'],
        title: 'Background Videos',
        description: 'Remove background videos',
        selected: false,
        size: 0,
        count: 0
    },
    background_image: {
        id: 'background_image',
        icon: CategoryIconComponents['background_image'],
        title: 'Background Images',
        description: 'Remove background images',
        selected: false,
        size: 0,
        count: 0
    },
    storyboard: {
        id: 'storyboard',
        icon: CategoryIconComponents['storyboard'],
        title: 'Storyboards',
        description: 'Remove storyboards',
        selected: false,
        size: 0,
        count: 0
    },
    hitsound: {
        id: 'hitsound',
        icon: CategoryIconComponents['hitsound'],
        title: 'Hitsounds',
        description: 'Remove hitsounds',
        selected: false,
        size: 0,
        count: 0
    },
    skin_element: {
        id: 'skin_element',
        icon: CategoryIconComponents['skin_element'],
        title: 'Skin Elements',
        description: 'Remove skin elements',
        selected: false,
        size: 0,
        count: 0
    }
}


const createCategoryState = () => {
    let categories = $state<CategoryState>(initialState);

    const reset = () => {
        categories = initialState;
    };

    const updateCategory = (sumarry: CategorySummaryResponse) => {
        for (let key in sumarry) {
            let keyCategory = key as keyof CategorySummaryResponse;
            categories[keyCategory].size = sumarry[keyCategory].total_size;
            categories[keyCategory].count = sumarry[keyCategory].total_count;
        }
    };

    const toggleCategorySelected = (id: string) => {
        categories[id].selected = !categories[id].selected;
    };

    return {
        get categories() {
            return categories;
        },
        set categories(value: CategoryState) {
            categories = value;
        },
        reset,
        updateCategory,
        toggleCategorySelected
    };
}

export default createCategoryState;