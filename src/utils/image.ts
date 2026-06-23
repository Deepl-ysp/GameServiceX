import { invoke } from "@tauri-apps/api/core"

export interface ImageObject {
    name: string;
    path: string;
    data: string;
    types: string;
}

export async function getImage(url: string): Promise<ImageObject> {
    const data:ImageObject = await invoke("readImage", { resPath: url });
    data.data = `data:image/png;base64,${data.data}`;
    return data;
}