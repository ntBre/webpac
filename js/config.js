let data = {
    program: "mopac",
    queue: "local",
    findiff: false,
};

const fields = [
    "geometry",
    "charge",
    "step_size",
    "template",
    "sleep_int",
    "job_limit",
    "chunk_size",
    "check_int",
];

const button = document.querySelector('button');

button.addEventListener('click', onClick);

function onClick() {
    for (const field of fields) {
	let val = document.getElementsByName(field)[0];
	if (val) {
	    data[field] = val.value;
	}
    }

    let ctype = Array.from(document.getElementsByName("coord_type")).filter(
	(t) => t.checked
    )[0];
    if (ctype) {
	data.coord_type = ctype.value;
    }
    data.optimize = document.getElementsByName("opt")[0].checked;

    data.charge = parseInt(data.charge, 10);
    data.step_size = parseFloat(data.step_size);
    data.sleep_int = parseInt(data.sleep_int, 10);
    data.job_limit = parseInt(data.job_limit, 10);
    data.chunk_size = parseInt(data.chunk_size, 10);
    data.check_int = parseInt(data.check_int, 10);

    const dataToSend = JSON.stringify(data);
    let req = {
	method: "post",
	headers: { "Content-Type": "application/json" },
	body: dataToSend
    };
    fetch("/run", req).then((res) => console.log(res));
}
