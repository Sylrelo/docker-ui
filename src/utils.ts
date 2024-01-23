import { invoke } from "@tauri-apps/api";

export async function dockerRequest(method: string, endpoint: string): Promise<any> {
  try {
    const result: any = await invoke("socket_communication", {
      method,
      endpoint,
    });

    // console.log({ result: result })

    return JSON.parse(result.data);
  } catch (e) {
    console.error(e);
    return {};
  }
}

export async function sleep(ms: number) {
  return new Promise((resolve, _) => {
    setTimeout(() => {
      resolve(null);
    }, ms)
  })
}

