import { totalColors } from "$lib/stores/state";

export async function get() {

    let colors: number;

    totalColors.subscribe(val => colors = val)();

    const randomId = Math.floor((Math.random() * (colors - 1))) + 1;

    return {
        headers: { Location: '/detail/' + randomId },
        status: 302
    }
}