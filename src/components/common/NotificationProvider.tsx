import React, { createContext, useContext, useRef } from 'react';
import { Toast } from 'primereact/toast';

interface NotificationContextType {
    showSuccess: (message: string, summary?: string) => void;
    showError: (message: string, summary?: string) => void;
    showInfo: (message: string, summary?: string) => void;
    showWarn: (message: string, summary?: string) => void;
}

const NotificationContext = createContext<NotificationContextType | undefined>(undefined);

export const NotificationProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
    const toast = useRef<Toast>(null);

    const showSuccess = (message: string, summary: string = 'Success') => {
        toast.current?.show({ severity: 'success', summary, detail: message, life: 3000 });
    };

    const showError = (message: string, summary: string = 'Error') => {
        toast.current?.show({ severity: 'error', summary, detail: message, life: 5000 });
    };

    const showInfo = (message: string, summary: string = 'Info') => {
        toast.current?.show({ severity: 'info', summary, detail: message, life: 3000 });
    };

    const showWarn = (message: string, summary: string = 'Warning') => {
        toast.current?.show({ severity: 'warn', summary, detail: message, life: 4000 });
    };

    return (
        <NotificationContext.Provider value={{ showSuccess, showError, showInfo, showWarn }}>
            <Toast ref={toast} />
            {children}
        </NotificationContext.Provider>
    );
};

export const useNotification = () => {
    const context = useContext(NotificationContext);
    if (!context) {
        throw new Error('useNotification must be used within a NotificationProvider');
    }
    return context;
};
