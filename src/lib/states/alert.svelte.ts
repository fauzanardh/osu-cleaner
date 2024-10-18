import type { AlertMessage } from '$lib/utils/interfaces';

const createAlertState = () => {
    let alerts = $state<AlertMessage[]>([]);

    const dismiss = (id: string) => {
        alerts = alerts.filter((alert) => alert.id !== id);
    };

    const show = (alert: Omit<AlertMessage, 'id'>) => {
        const id = crypto.randomUUID();
        alerts = [...alerts, { ...alert, id }];

        if (!alert.persist && alert.timeout !== 0) {
            setTimeout(() => {
                dismiss(id);
            }, alert.timeout || 3000);
        }
    };

    const clear = () => {
        alerts = [];
    };

    return {
        get alerts() {
            return alerts;
        },
        show,
        dismiss,
        clear
    }
};

export default createAlertState;