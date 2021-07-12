function pathToHole(points) {
	let hole = '';
	let op = 'M ';
	let bounds = getBounds(points);
	let x1 = bounds[0][0]-5;
	let y1 = bounds[0][1]-5;
	let x2 = bounds[1][0]+5;
	let y2 = bounds[1][1]+5;

	for(const point of points){
		hole += op + point[0] + "," + point[1];
		op = ' L ';
	}
	return hole + ' Z M '+x1+','+y1+' L '+x2+','+y1+' L '+x2+','+y2+' L '+x1+','+y2;
}

function setElementPaths(element, edges, vertices) {
	while(element.firstChild) {
		element.removeChild(element.firstChild);
	}

	for(const edge of edges){
		let path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		let p1 = vertices[edge[0]]
		let p2 = vertices[edge[1]];
		path.setAttribute('d', 'M ' + p1[0] + ',' + p1[1] + ' L ' + p2[0] + ',' + p2[1]);
		element.appendChild(path);
	}
}

function parseSolutionData(problemData, solutionString) {
	let solutionData = {}
	let errorList = document.getElementById('solution-errors');
	while(errorList.firstChild) {
		errorList.removeChild(errorList.firstChild);
	}

	try {
		solutionData = JSON.parse(solutionString);
	} catch (error) {
		console.error("Could not parse solution.");
		console.error(error);
		let li = document.createElement('li');
		li.textContent = "" + error;
		errorList.appendChild(li);
		throw error;
	}
	return solutionData;
}

function resetSolution() {
	document.getElementById('select-solution').value = 'CUSTOM';
	let problemData = JSON.parse(document.getElementById('problem-data').value);
	document.getElementById('solution-data').value = JSON.stringify({"vertices":problemData.figure.vertices});
}

function squaredDistance(p, q) {
	return ((p[0] - q[0]) ** 2) + ((p[1] - q[1]) ** 2);
}

function calculateRatio(oldEdge, newEdge) {
	return Math.abs((squaredDistance(newEdge[0], newEdge[1]) / squaredDistance(oldEdge[0], oldEdge[1])) - 1);
}

function possiblePoints(numVertices, numEdges, numHoleVertices){
	return Math.round(1000 * Math.log2(numVertices, numEdges, numHoleVertices));
}

function calculateDislikes(hole, vertices){
	let dislikes = 0;
	for(const hv of hole) {
		let min = Number.MAX_SAFE_INTEGER;
		for(const v of vertices) {
			let d = squaredDistance(hv, v);
			if(d < min){
				min = d;
			}
		}
		dislikes += min;
	}
	return Math.round(dislikes);
}

function linesCross(line1, line2) {
	let width1 = line1[1][0] - line1[0][0];
	let height1 = line1[1][1] - line1[0][1];
	let width2 = line2[1][0] - line2[0][0];
	let height2 = line2[1][1] - line2[0][1];
	let s = (-height1 * (line1[0][0] - line2[0][0]) + width1 * (line1[0][1] - line2[0][1])) / (-width2 * height1 + width1 * height2);
	let t = ( width2 * (line1[0][1] - line2[0][1]) - height2 * (line1[0][0] - line2[0][0])) / (-width2 * height1 + width1 * height2);
	return s > 0 && s < 1 && t > 0 && t < 1;
}

function lineCrossesBounds(line, bounds){
	let j = bounds.length - 1;
	for (var i = 0; i < bounds.length; i++) {
		if(linesCross(line, [bounds[j],bounds[i]])){
			return true;
		}
		j = i;
	}
	return false;
}

function pointInside(point, vertices) {
	let x = point[0];
	let y = point[1];
	let crossings = 0;

	// Odd number of crossings is inside
	let j = vertices.length - 1;
	for (var i = 0; i < vertices.length; i++) {
		let xi = vertices[i][0];
		let yi = vertices[i][1];
		let xj = vertices[j][0];
		let yj = vertices[j][1];
		
		// On a point is inside
		if(x==xi && y==yi){
			return true;
		}

		if(((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi)){
			crossings += 1;
		}
		j = i;
	}

	return (crossings%2)==1;
};

function callHilightLine(element, i){
	element.onmouseover = function(e){hilightLine(i);};
}

function hilightLine(i){
	edge = problemData.figure.edges[i];
	
	let g = document.getElementById('hilight-paths');
	while(g.firstChild) {
		g.removeChild(g.firstChild);
	}
	p1 = solutionData.vertices[edge[0]];
	p2 = solutionData.vertices[edge[1]];
	
	try {
		let path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path.setAttribute('d', 'M ' + p1[0] + ',' + p1[1] + ' L ' + p2[0] + ',' + p2[1]);
		g.appendChild(path);
	} catch(e) {
		console.error(e);
	}
}

function callHilightBoundry(element, i){
	element.onmouseover = function(e){hilightBoundry(i);};
}

function hilightBoundry(i){
	j = i==0 ? problemData.hole.length-1 : i-1;
	p1 = problemData.hole[i];
	p2 = problemData.hole[j];
	
	let g = document.getElementById('hilight-boundaries');
	while(g.firstChild) {
		g.removeChild(g.firstChild);
	}
	
	try {
		let path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path.setAttribute('d', 'M ' + p1[0] + ',' + p1[1] + ' L ' + p2[0] + ',' + p2[1]);
		g.appendChild(path);
	} catch(e) {
		console.error(e);
	}
}

function hilightPoint(i, vertex){
	edge = problemData.figure.edges[i];
	
	let g = document.getElementById('hilight-points');
	while(g.firstChild) {
		g.removeChild(g.firstChild);
	}
	point = solutionData.vertices[edge[vertex]];
	
	try {
		let path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
		path.setAttribute('d', 'M ' + point[0] + ',' + point[1] + ' L ' + point[0] + ',' + point[1]);
		g.appendChild(path);
	} catch(e) {
		console.error(e);
	}
}

function updateSolution(id){
	let input = document.getElementById(id);
	let i = input.dataset.i;
	let vertex = input.dataset.vertex;
	let coord = input.dataset.coord;
	let edge = problemData.figure.edges[i]
	solutionData.vertices[edge[vertex]][coord] = parseInt(input.value);
	document.getElementById('solution-data').value = JSON.stringify(solutionData);
	redraw();
}

function createPointInput(i, vertex, coord, value){
	let name = 'point-'+i+'-'+vertex+'-'+coord;
	let input = document.createElement('input');
	input.id = name;
	input.setAttribute('type', 'number');
	input.onfocus = function(e){hilightPoint(i, vertex);};
	input.onmouseover = function(e){hilightPoint(i, vertex);};
	input.onchange = function(e){updateSolution(name);};
	input.dataset.i = i;
	input.dataset.vertex = vertex;
	input.dataset.coord = coord;
	input.name = name;
	input.value = value;
	input.size = 3;
	input.classList.add('number-input');
	return input;
}

function populateEpsilonTable(table, epsilon, edges, oldVertices, vertices, hole) {
	let maxRatio = epsilon/1000000;
	let vertexStatus = true;
	let crossStatus = true;
	let epsilonStatus = true;

	while(table.firstChild) {
		table.removeChild(table.firstChild);
	}

	let errorPaths = document.getElementById('error-paths');
	while(errorPaths.firstChild) {
		errorPaths.removeChild(errorPaths.firstChild);
	}

	document.getElementById('epsilon').textContent = epsilon;
	document.getElementById('max-ratio').textContent = maxRatio;
	document.getElementById('possible-points').textContent = possiblePoints(vertices.length, edges.length, hole.length);
	document.getElementById('dislikes').textContent = calculateDislikes(hole, vertices);

	let i=0;
	for(const edge of edges){
		let p1 = vertices[edge[0]]
		let p2 = vertices[edge[1]];
		let oldP1 = oldVertices[edge[0]]
		let oldP2 = oldVertices[edge[1]];
		let newLength = squaredDistance(p1, p2);
		let oldLength = squaredDistance(oldP1, oldP2);
		let ratio = calculateRatio([oldP1, oldP2], [p1, p2]);
		let isInside = pointInside(p1, hole) && pointInside(p2, hole);
		let isCrossBounds = lineCrossesBounds([p1, p2], hole);

		let tr = document.createElement('tr');
		if(!isInside || isCrossBounds || ratio > maxRatio){
			if(!isInside){
				vertexStatus = false;
			}
			if(isCrossBounds){
				crossStatus = false;
			}
			if(ratio > maxRatio){
				epsilonStatus = false;
			}
			tr.classList.add('point-error');

			let path = document.createElementNS('http://www.w3.org/2000/svg', 'path');
			path.setAttribute('d', 'M ' + p1[0] + ',' + p1[1] + ' L ' + p2[0] + ',' + p2[1]);
			errorPaths.appendChild(path);
		}

		let td = document.createElement('td');
		td.textContent = i;
		tr.appendChild(td);

		td = document.createElement('td');
		td.textContent = isInside ? '🟢' : '❌';
		tr.appendChild(td);
		tr.add

		td = document.createElement('td');
		td.textContent = isCrossBounds ? '❌' : '🟢';
		tr.appendChild(td);
		tr.add

		td = document.createElement('td');
		td.appendChild(createPointInput(i, 0, 0, p1[0]));
		tr.appendChild(td);

		td = document.createElement('td');
		td.appendChild(createPointInput(i, 0, 1, p1[1]));
		tr.appendChild(td);

		td = document.createElement('td');
		td.appendChild(createPointInput(i, 1, 0, p2[0]));
		tr.appendChild(td);

		td = document.createElement('td');
		td.appendChild(createPointInput(i, 1, 1, p2[1]));
		tr.appendChild(td);

		td = document.createElement('td');
		td.textContent = oldLength;
		tr.appendChild(td);

		td = document.createElement('td');
		td.textContent = newLength;
		tr.appendChild(td);

		td = document.createElement('td');
		td.textContent = ratio.toFixed(5);
		if(ratio>maxRatio){
			td.classList.add('ratio-error');
		}
		tr.appendChild(td);
		callHilightLine(tr, i);

		table.appendChild(tr);
		i++;
	}

	updateStatusCell('vertex-status', vertexStatus);
	updateStatusCell('cross-status', crossStatus);
	updateStatusCell('epsilon-status', epsilonStatus);
}

function populateHoleTable(table, vertices) {
	while(table.firstChild) {
		table.removeChild(table.firstChild);
	}

	let j = vertices.length - 1;
	for (var i = 0; i < vertices.length; i++) {
		let p1 = vertices[i];
		let p2 = vertices[j];
		let length = squaredDistance(p1, p2);

		let tr = document.createElement('tr');

		for(const v of [i, length, p1[0], p1[1], p2[0], p2[1]]){
			let td = document.createElement('td');
			td.textContent = v;
			tr.appendChild(td);
		}

		table.appendChild(tr);
		callHilightBoundry(tr, i);
		j = i;
	}
}

function updateStatusCell(id, status){
	let el = document.getElementById(id);
	if(status){
		el.textContent = 'PASS';
		el.classList.remove('fail');
		el.classList.add('success');
	} else {
		el.textContent = 'FAIL';
		el.classList.remove('success');
		el.classList.add('fail');
	}
}

function getBounds(points) {
	let minX = Number.MAX_SAFE_INTEGER;
	let minY = Number.MAX_SAFE_INTEGER;
	let maxX = 0;
	let maxY = 0;

	for(const point of points) {
		minX = Math.min(minX, point[0]);
		minY = Math.min(minY, point[1]);
		maxX = Math.max(maxX, point[0]);
		maxY = Math.max(maxY, point[1]);
	}
	return [[minX, minY],[maxX, maxY]];
}

let problemData = {};
let solutionData = {};
function redraw() {
	let svg = document.getElementById('vis');
	let hole = document.getElementById('hole');
	let figure = document.getElementById('figure');
	let solution = document.getElementById('solution');
	problemData = JSON.parse(document.getElementById('problem-data').value);

	let errorPaths = document.getElementById('error-paths');
	while(errorPaths.firstChild) {
		errorPaths.removeChild(errorPaths.firstChild);
	}
	let hilightPaths = document.getElementById('hilight-paths');
	while(hilightPaths.firstChild) {
		hilightPaths.removeChild(hilightPaths.firstChild);
	}

	try {
		solutionData = parseSolutionData(problemData, document.getElementById('solution-data').value);
		setElementPaths(solution, problemData.figure.edges, solutionData.vertices);
	} catch(error) {
		return;
	}

	let points = problemData.hole.slice();
	points = points.concat(problemData.figure.vertices);
	points = points.concat(solutionData.vertices);
	let bounds = getBounds(points);
	let x1 = bounds[0][0]-5;
	let y1 = bounds[0][1]-5;
	let x2 = (-x1) + bounds[1][0]+10;
	let y2 = (-y1) + bounds[1][1]+10;
	svg.setAttributeNS(null, 'viewBox', '' + x1 + ' ' + y1 + ' ' + x2 + ' ' + y2);

	hole.setAttribute('d', pathToHole(problemData.hole));
	setElementPaths(figure, problemData.figure.edges, problemData.figure.vertices);

	populateEpsilonTable(document.getElementById('epsilon-table-body'), problemData.epsilon, problemData.figure.edges, problemData.figure.vertices, solutionData.vertices, problemData.hole);
	populateHoleTable(document.getElementById('hole-table-body'), problemData.hole);
}

function loadProblemFile(){
	let file = document.getElementById('problem-file').files[0];
	let reader = new FileReader();
	let solutionName = file.name;
	document.getElementById('solution-save-link').setAttribute('download', solutionName);
	document.getElementById('solution-save').textContent = 'Save ' + solutionName;
	reader.onload = function(e) {
		document.getElementById('problem-data').value = e.target.result;
		resetSolution();
		redraw();
	}
	reader.readAsText(file);
}

function loadSolutionFile(){
	let file = document.getElementById('solution-file').files[0];

	let reader = new FileReader();
	reader.onload = function(e) {
		document.getElementById('solution-data').value = e.target.result;
		redraw();
	}
	reader.readAsText(file);
}

var saveFile = null;
function saveSolution(){
	var data = new Blob([document.getElementById('solution-data').value], {type: 'text/json'});
	if(saveFile !== null) {
		window.URL.revokeObjectURL(saveFile);
	}
	saveFile = window.URL.createObjectURL(data);

	var link = document.getElementById('solution-save-link');
	link.href = saveFile;

	link.dispatchEvent(new MouseEvent('click'));
}

function loadProblemList(){
	fetch('problems/list')
		.then(response => response.json())
		.then(data => {
			populateFileList(data.files, 'select-problem', 'Problem ');
		});
}

function loadSolutionList(){
	fetch('solutions/list')
		.then(response => response.json())
		.then(data => {
			populateFileList(data.files, 'select-solution', 'Solution ');
		});
}

function loadProblemURL(){
	let fileName = document.getElementById('select-problem').value;
	if(fileName=='CUSTOM'){
		return;
	}
	fetch('problems/'+fileName)
		.then(response => response.json())
		.then(data => {
			document.getElementById('problem-data').value = JSON.stringify(data);
			solutions = document.getElementById('select-solution').childNodes;
			solutions.value = fileName;
			let found = false;
			for(const option of solutions){
				if(option.value == fileName){
					document.getElementById('select-solution').value = fileName;
					loadSolutionURL()
					found = true;
					break;
				}
			}
			if(!found){
				resetSolution();
				redraw();
			}
		});
}

function loadSolutionURL(){
	let fileName = document.getElementById('select-solution').value;
	if(fileName=='CUSTOM'){
		return;
	}
	fetch('solutions/'+fileName)
		.then(response => response.json())
		.then(data => {
			document.getElementById('solution-data').value = JSON.stringify(data);
			redraw();
		});
}


function populateFileList(files, id, prefix){
	let select = document.getElementById(id);

	let custom = null;
	while(select.childNodes.length > 2) {
		let child = select.firstChild;
		if(child.value=='CUSTOM'){
			custom = child;
		}
		select.removeChild(select.lastChild);
	}
	if(custom){
		select.appendChild(custom);
	}

	sortedFiles = [];
	for(const file of files){
		let p1 = file.split('.')[0];
		let n1 = parseInt(p1);
		for(var i=0; i<sortedFiles.length; i++){
			p2 = sortedFiles[i].split('.')[0];
			let n2 = parseInt(p2);
			if(n1 < n2){
				break;
			}
		}
		sortedFiles.splice(i, 0, file);
	}

	let first = true;
	for(const file of sortedFiles){
		let option = document.createElement('option');
		option.textContent = prefix + file;
		option.value = file;
		select.appendChild(option);
		if(first){
			first = false;
			select.value = file;
		}
	}
}

let solverRunning = false;
function selectSolver(){
	stopSolver();
}

let solverData = "";
function runSolver(){
	if(!solverRunning){
		let problem = document.getElementById('select-problem').value.split('.',1)[0];
		let solver = document.getElementById('select-solver').value;
		if(solver=='BIONEURAL'){
			alert("This means you. Start editing...");
			return;
		}
		solverRunning = true;
		setTimeout(fetchStatus(), 1000);
		fetch('/problem/'+problem+'/solve/'+solver)
			.then(response => response.json())
			.then(data => {
				console.log("Solve complete:", data);
				solverData = data;
				lines = solverData.output.trim().split("\n");
				document.getElementById('solution-data').value = lines[lines.length-1];
				solverRunning = false;
				redraw();
		});
	}
}

function fetchStatus(){
	console.log("Fetching status...");
	fetch('/queue/pop')
		.then(response => response.json())
		.then(data => {
			if(data.done===false){
				if(data.line){
					document.getElementById('solution-data').value = data.line;
					redraw();
				}
				fetchStatus();
			}else if(solverRunning){
				setTimeout(fetchStatus, 1000);
			}
		});
}

function stopSolver(){
	if(solverRunning){
		console.log("TODO: Stop the solver.");
		solverRunning = false
	}
}


redraw();
loadProblemList();
loadSolutionList();
