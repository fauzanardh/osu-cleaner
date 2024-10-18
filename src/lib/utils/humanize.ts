function humanizeFileSize(size: number) {
    if (!size || size < 0 || !Number.isFinite(size)) {
        return '0 B';
    }
    const units = ['B', 'kB', 'MB', 'GB', 'TB'];
    const i = size === 0 ? 0 : Math.floor(Math.log(size) / Math.log(1024));
    const unitIndex = Math.min(i, units.length - 1);
    const formattedSize = (size / Math.pow(1024, unitIndex)).toFixed(2);
    return `${formattedSize} ${units[unitIndex]}`;
}

export { humanizeFileSize };