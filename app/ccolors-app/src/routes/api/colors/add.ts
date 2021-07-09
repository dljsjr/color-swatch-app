export async function post(request) {
    const payload = request.body;

    const response = await fetch("http://api:8000/colors/add", {
        method: 'POST',
        headers: {
            'Accept': 'application/json',
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    })

    if (response.ok) {
        return {
            body: {
                json: await response.json()
            }
        };
    } else {
        return {
            status: response.status,
            error: new Error(`Error adding color ${await response.json()}`)
        };
    }
}