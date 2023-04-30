
function renderItems(items, processType, elementId, processFn) {
	console.log('rendering items');
	let itemsMeta = [];

	let placeholder = '<div>';
	items.forEach(item => {
		let title = item['title'];
		let placeholderId = processType + "-" + title.replaceAll(" ", "-");
		placeholder += `
			<div>title
				<button id="${placeholderId}>
					${processType}
				</button>
			</div>
		`
		itemsMeta.push({
			'id': placeholderId,
			'title': title,
		});
	});
	placeholder += '</div>';

	document
		.getElementById(elementId)
		.innerHTML = placeholder;

	itemsMeta.forEach(item => {
		document
			.getElementById(item['id'])
			.addEventListener('click', processFn);
	});
}

function webApiRequest(url, method) {
	console.log('xml http request');
	let xhr = new XMLHttpRequest();
	xhr.withCredentials = true;
	xhr.addEventListener('readystatechange', function(){
		if (this.readyState === this.DONE) {
			renderItems(
				JSON.parse(this.responseText)['pending_items'],
				'edit', 'pending-items', editItem
			);
			renderItems(
				JSON.parse(this.responseText)['done_items'],
				'delete', 'done-items', deleteItem 
			);
		}
	});

	xhr.open(method, url);
	xhr.setRequestHeader('content-type', 'application-json');
	xhr.setRequestHeader('user-token', 'token');
	return xhr;
}

function editItem() {
	let title = this.id.replaceAll('-', ' ').replace('edit ','');
	let request = webApiRequest('/v1/item/edit', 'POST');
	let body = {
		"title": title,
		"status": "DONE"
	};
	request.send(JSON.stringify(json));
}

function deleteItem() {
	let title = this.id.replaceAll('-', ' ').replace('delete','');
	let request = webApiRequest('/v1/item/delete', 'POST');
	let body = {
		"title": title,
		"status": "DONE"
	};
	request.send(JSON.stringify(json));
}

function getItems() {
	console.log('get items');
	webApiRequest('/v1/item', 'GET').send();
}
getItems();

function createItem() {
	console.log('create');
	let title = document.getElementById('name');
	webApiRequest(`/v1/item/create/${title.value}`, 'POST').send();
	title.value = null;
}
