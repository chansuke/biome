/* should not generate diagnostics */

import React from "react";

[<Hello key="first" />, <Hello key="second" />, <Hello key="third" />];

[...[<Hello key="first" />, <Hello key="second" />], <Hello key="third" />];

[<Hello key="first" />, xyz ? <Hello key="second"/>: <Hello key="second" />, <Hello key="third" />];

[<React.Fragment key="first"></React.Fragment>, <React.Fragment key="second"></React.Fragment>, <React.Fragment key="third"></React.Fragment>];

data.map(x => <Hello key={x}>{x}</Hello>);

data.map(x => <React.Fragment key={x}>{x}</React.Fragment>);

data.forEach(x => data1.push(<React.Fragment key={x}>{x}</React.Fragment>));

Array.from([1, 2, 3], (x) => <Hello key={x}>{x}</Hello>);

Array.from([1, 2, 3], (x) => {
	return <Hello key={x}>{x}</Hello>
});

[React.createElement("h1", {key}), React.createElement("h1", {key: "second"}), React.createElement("h1", {key: third})];

data.map(c => React.createElement("h1", {key}));

React.Children.map(c => React.cloneElement(c, {key: c}));

(<h1></h1>)

(<h1>
		<h2></h2>
		<h2></h2>
		<h2></h2>
		<h2></h2>
</h1>)

(<h1>{data.map(x => <h1 key={x}>{x}</h1>)}</h1>)

(<h1>{[<h1 key={1}></h1>, <h1 key={2}></h1>, <h1 key={3}></h1>]}</h1>)

(<h1>{data.map(c => <h1 key={c}></h1>)}</h1>)

(<h1>{data.map(c => (<h1 key={c}></h1>))}</h1>)

(<h1>{data.map(c => {return (<h1 key={c}></h1>)})}</h1>)

<>{data.reduce((total, next) => total + next, 0)}</>

<>{data.reduce((a, b) => Math.max(a, b), 0)}</>

<>{data.reduce((a, b) => a > b ? a : b, 0)}</>

<>{data.map(a => a > 4 ? <h1 key={a}>{a}</h1> : <h2 key={a}>{a}</h2>)}</>

[].map((item) => {
	return <><div  key={item.id} /><div key={item.id}>{item}</div></>;
});

[].map((item) => {
	const div = <div key={item.id} />;
	return <>{div}<div key={item.id}>{item}</div></>;
});

[].map((item) => {
	return <><div key={item.id}>foo</div></>;
});

[].map((item) => {
	return item.condition ? <div key={item.id} /> : <div key={item.id}>foo</div>;
});

[].map((item) => {
	return <>{item.condition ? <div key={item.id} /> : <div key={item.id}>foo</div>}</>;
});

[].map((item) => {
	const div = <div key={item.id} />;
	return <>{item.condition ? div : <div key={item.id}>{div}</div>}</>;
});

[].map(function(item) {
	const x = 5;
	return <div key={item.id}>{x}</div>;
});

[].map(function(item) {
	const x = 5;
	const div = <div key={item.id}>{x}</div>;
	return div;
});
