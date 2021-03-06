import { BASE_URL } from '../constants';
import { Light, HooLight } from '../types/light';

export async function getAllLights(): Promise<HooLight[]> {
    const url = `${BASE_URL}/lights`;
    const response = await fetch(url);
    const lights: HooLight[] = await response.json();
    return lights;
}

export async function getLight(lightNumber: number): Promise<HooLight> {
    const url = `${BASE_URL}/light/${lightNumber}`;
    const response: any = await fetch(url);
    const light: HooLight = await response.json();
    return light;
}

export async function on(lightNumber: number) {
    const url = `${BASE_URL}/light/${lightNumber}/on`;
    await fetch(url, { method: 'PUT' });
}

export async function off(lightNumber: number) {
    const url = `${BASE_URL}/light/${lightNumber}/off`;
    await fetch(url, { method: 'PUT' });
}

export async function setBrightness(lightNumber: number, brightness: number) {
    const url = `${BASE_URL}/light/${lightNumber}/state?bri=${brightness}`;
    await fetch(url, { method: 'PUT' });
}

export async function setSaturation(lightNumber: number, saturation: number) {
    const url = `${BASE_URL}/light/${lightNumber}/state?sat=${saturation}`;
    await fetch(url, { method: 'PUT' });
}

export async function setHue(lightNumber: number, hue: number) {
    const url = `${BASE_URL}/light/${lightNumber}/state?hue=${hue}`;
    await fetch(url, { method: 'PUT' });
}
