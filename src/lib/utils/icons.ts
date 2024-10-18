import { FileMusic, FileQuestion, Image, Images, ScrollText, Video } from 'lucide-svelte';
import type { ComponentType } from 'svelte';

const CategoryIconComponents: Record<string, ComponentType> = {
    background_video: Video,
    background_image: Image,
    storyboard: ScrollText,
    hitsound: FileMusic,
    skin_element: Images,
    other: FileQuestion
};

export { CategoryIconComponents };