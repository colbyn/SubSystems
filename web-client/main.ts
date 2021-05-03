// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
// const rust = import('./pkg');

interface Tag {
    tag: string,
    attrs: Object,
    apply?: (element: Element) => void,
    children: Array<Html>
}

type Html = Tag | string;

function instanceOfElement(object: any): object is Tag {
    if (!instanceOfText(object)) {
        return 'tag' in object;
    } else {
        return false;
    }
}
function instanceOfText(object: any): object is Tag {
    return (typeof object === 'string' || (object as any) instanceof String)
}


function tag(
    tag: string,
    attrs: Object,
    children: Array<Html>,
    apply?: (element: HTMLElement) => void
): Tag {
    return {tag: tag, attrs: attrs, children: children, apply: apply}
}

function layout_html(html: Html): Node {
    if (instanceOfElement(html)) {
        let node = document.createElement(html.tag);
        for (const child of html.children) {
            node.appendChild(layout_html(child));
        }
        for (const key of Object.keys(html.attrs)) {
            const value = (html.attrs as any)[key];
            if ((typeof value === 'string') || ((value as any) instanceof String)) {
                node.setAttribute(key, value);
            } else {
                node.setAttribute(key, JSON.stringify(value));
            }
        }
        if (html.apply) {
            html.apply(node);
        }
        return node as Node;
    } else if (instanceOfText(html)) {
        return document.createTextNode(html) as Node;
    } else {
        throw "Invalid Node"
    }
}

function uuid(): string {
    return 'UID_xxxxxxxx_xxxx_4xxx_yxxx_xxxxxxxxxxxx'.replace(/[xy]/g, function(c) {
        var r = Math.random() * 16 | 0, v = c == 'x' ? r : (r & 0x3 | 0x8);
        return v.toString(16);
    });
}

function add_form_entry_function(
    add_error: (msg: string)=>void,
    code_str: string,
    info_str: string
) {
    const rust = import('./pkg/index');
    const uid = uuid();
    const get_input_value = (form: HTMLElement): string => {
        return (form.querySelector('input[type="text"]') as HTMLInputElement).value;
    };
    let node = tag('div', {'form-wrapper': '', id: uid}, [
        tag('p', {}, [info_str]),
        tag(
            'form',
            {id: uid},
            [
                tag('input', {type: 'text'}, [], (element) => {
                    let input = element as HTMLInputElement;
                    input.value = code_str;
                }),
                tag('input', {type: 'submit'}, []),
            ],
            (element) => {
                element.onsubmit = (event) => {
                    rust.then(module => {
                        const input = get_input_value(element);
                        const result = module.chem_eval(input) as any;
                        const ast_str = result['ast'];
                        const value_str = result['value'];
                        let output = document.querySelector(`#${uid} > [output]`);
                        const new_output = layout_html(tag('table', {}, [
                            tag('tr', {}, [
                                tag('td', {}, ["AST TREE"]),
                                tag('td', {}, [
                                    tag('pre', {}, [ast_str])
                                ]),
                            ]),
                            tag('tr', {}, [
                                tag('td', {}, ["VALUE"]),
                                tag('td', {}, [
                                    tag('code', {}, [value_str])
                                ]),
                            ]),
                        ])) as Element;
                        output.parentElement.replaceChild(new_output, output);
                    
                    }).catch(add_error);
                    console.log('done');
                    return false;
                };
            },
        ),
        tag('div', {output: ''}, []),
    ]);
    document.body.appendChild(layout_html(node));
}

function add_form_entry_reaction(
    add_error: (msg: string)=>void,
    code_str: string,
    info_str: string
) {
    const rust = import('./pkg/index');
    const uid = uuid();
    const get_input_value = (form: HTMLElement): string => {
        return (form.querySelector('input[type="text"]') as HTMLInputElement).value;
    };
    let node = tag('div', {'form-wrapper': '', id: uid}, [
        tag('p', {}, [info_str]),
        tag(
            'form',
            {id: uid},
            [
                tag('input', {type: 'text'}, [], (element) => {
                    let input = element as HTMLInputElement;
                    input.value = code_str;
                }),
                tag('input', {type: 'submit'}, []),
            ],
            (element) => {
                element.onsubmit = (event) => {
                    rust.then(module => {
                        const input = get_input_value(element);
                        const result = module.balance_reaction(input) as any;
                        const input_ast = result['input_ast'];
                        const input_str = result['input_str'];
                        const output_ast = result['output_ast'];
                        const output_str = result['output_str'];
                        let output = document.querySelector(`#${uid} > [output]`);
                        const new_output = layout_html(tag('table', {}, [
                            tag('tr', {}, [
                                tag('td', {}, ["INPUT AST TREE"]),
                                tag('td', {}, [
                                    tag('pre', {}, [input_ast])
                                ]),
                            ]),
                            tag('tr', {}, [
                                tag('td', {}, ["INPUT VALUE"]),
                                tag('td', {}, [
                                    tag('code', {}, [input_str])
                                ]),
                            ]),
                            tag('tr', {}, [
                                tag('td', {}, ["OUTPUT AST TREE"]),
                                tag('td', {}, [
                                    tag('pre', {}, [output_ast])
                                ]),
                            ]),
                            tag('tr', {}, [
                                tag('td', {}, ["OUTPUT VALUE"]),
                                tag('td', {}, [
                                    tag('code', {}, [output_str])
                                ]),
                            ]),
                        ])) as Element;
                        output.parentElement.replaceChild(new_output, output);
                    
                    }).catch(add_error);
                    console.log('done');
                    return false;
                };
            },
        ),
        tag('div', {output: ''}, []),
    ]);
    document.body.appendChild(layout_html(node));
}


window.onload = function() {
    let errors = document.createElement('div');
    errors.setAttribute('app-errors', '');
    document.body.appendChild(errors);
    function add_error(msg: string) {
        console.error(msg);
        let node = document.createElement('p');
        let stying = `
            color: #131313;
            background-color: #ff57574a;
            padding: 12px;
            font-family: monospace;
        `;
        node.setAttribute('style', stying);
        node.innerText = msg;
        node.setAttribute('error', '');
        errors.appendChild(node);
    }
    console.log("loaded");
    document.body.appendChild(layout_html(tag('h1', {}, [
        "Chemical Functions"
    ])));
    add_form_entry_function(
        add_error,
        "mole(energy(photon(wavelength = nm(325))))",
        "energy of one mole of photons given wavelength"
    );
    add_form_entry_function(
        add_error,
        "energy(photon(wavelength = nm(325)))",
        "energy of photon given wavelength"
    );
    add_form_entry_function(
        add_error,
        "energy(photon(frequency = GHz(275)))",
        "energy of photon given frequency"
    );
    add_form_entry_function(
        add_error,
        "wavelength(frequency = MHz(72.5))",
        "wavelength from frequency"
    );
    add_form_entry_function(
        add_error,
        "frequency(wavelength = nm(325))",
        "frequency from wavelength"
    );
    document.body.appendChild(layout_html(tag('h1', {}, [
        "Chemical Reaction"
    ])));
    add_form_entry_reaction(
        add_error,
        "C3H8 + O2 -> CO2 + H2O",
        "balance chemical reactions"
    );
    add_form_entry_reaction(
        add_error,
        "Ca(O3H2)2(aq) + HCl(aq) -> CaCl2(aq) + H2O(l)",
        "balance chemical reactions"
    );
    add_form_entry_reaction(
        add_error,
        "PCl5 + H2O -> H3PO4 + HCl",
        "balance chemical reactions"
    );
    add_form_entry_reaction(
        add_error,
        "Al + O2 -> Al2O3",
        "balance chemical reactions"
    );
    add_form_entry_reaction(
        add_error,
        "Na3PO4 + Ba(NO3)2 -> Ba3(PO4)2 + NaNO3",
        "balance chemical reactions"
    );
    add_form_entry_reaction(
        add_error,
        "2XY2 + 4Y -> 8X2 + 8YX",
        "balance chemical reactions"
    );
};

