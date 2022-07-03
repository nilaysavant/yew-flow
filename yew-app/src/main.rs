use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct TodoItem {
    id: usize,
    value: String,
    checked: bool,
}

#[derive(Clone, Properties, PartialEq)]
struct RenderTodoItemProps {
    todo_item: TodoItem,
    on_click: Callback<TodoItem>,
}

#[function_component(RenderTodoItem)]
fn render_todo_item(
    RenderTodoItemProps {
        todo_item,
        on_click,
    }: &RenderTodoItemProps,
) -> Html {
    log::info!("render_todo_item: {}", todo_item.id);
    let TodoItem { id, value, checked } = todo_item;

    let handle_click = {
        let on_click = on_click.clone();
        let todo_item = todo_item.clone();
        Callback::from(move |_| on_click.emit(todo_item.clone()))
    };

    html! {
        <p
            onclick={handle_click}
            style={format!("user-select: none; cursor: pointer; text-decoration: {};", if *checked {"line-through"} else {"none"} )}
        >
            {value}
        </p>
    }
}

struct State {
    todos: Vec<TodoItem>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            todos: (0..5)
                .map(|id| TodoItem {
                    id,
                    value: format!("Todo Item: {}", id),
                    checked: false,
                })
                .collect(),
        }
    }
}

enum StateAction {
    AddTodo(TodoItem),
    ToggleTodo(usize),
}

impl Reducible for State {
    type Action = StateAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut todos = self.todos.clone();
        match action {
            StateAction::AddTodo(todo_item) => todos.push(todo_item),
            StateAction::ToggleTodo(id) => {
                let todo = todos.iter_mut().find(|t| t.id == id);
                if let Some(todo) = todo {
                    todo.checked = !todo.checked;
                }
            }
        };
        Self { todos }.into()
    }
}

#[function_component(App)]
fn app() -> Html {
    let store = use_reducer(State::default);

    let handle_todo_click = {
        let store = store.clone();
        Callback::from(move |todo: TodoItem| store.dispatch(StateAction::ToggleTodo(todo.id)))
    };

    let render_todos = store
        .todos
        .iter()
        .map(|todo| {
            html! {
                <RenderTodoItem key={todo.id} todo_item={todo.clone()} on_click={handle_todo_click.clone()} />
            }
        })
        .collect::<Html>();

    html! {
        <>
            {render_todos}
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
