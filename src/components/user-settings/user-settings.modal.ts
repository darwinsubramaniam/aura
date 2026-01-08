export interface UserSettings {
    locale: string;
    default_fiat_id: number;
}

export interface UpdateUserSettings {
    id: number;
    locale?: string;
    default_fiat_id?: number;
}
