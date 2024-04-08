use leptos::*;

/// When unit testing a component, the best thing to do is to extract the
/// logic from the macro and place it in it's own Rust type. This type can
/// then be tested normally.
#[component]
pub fn ToDoApp() -> impl IntoView {
    
    // create a signal to listen to a Struct for changes
    let (todos, set_todos) = create_signal(ToDos(Vec::new()));

    // listen to an input box for changes
    let input_element: NodeRef<html::Input> = create_node_ref();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        let value = input_element.get().expect("<input> should be mounted").value();
        set_todos.update(|todos| todos.new_todo(value));
    };

    view! {
        
        <form on:submit=on_submit>
            <input type="text"
                value="task"
                node_ref=input_element
            />
            <input type="submit" value="Submit"/>
        </form>

        // `todos.with` is a lot like `todos.get` in that it listens for a change
        // in the tods signal. Instead of returning a primitive value though, it returns
        // the return value of some custom logic.
        <p>"To Dos remaining: " {move || todos.with(|todos| todos.num_remaining())}</p>

        // list the todos
        <ul>
            {
                move || todos.with(|todos| {
                    todos.get()
                })
                    .into_iter()
                    .map(|todo| view! { <li>{todo.title}</li> })
                    .collect_view()
            }
        </ul>
    }

}


// Enclose the logic of the component in these structs to keep the component
// minimal and focussed only on rendering HTML tags.
struct ToDos(Vec<ToDo>);

impl ToDos {
    fn num_remaining(&self) -> usize {
        self.0.iter().filter(|todo| !todo.completed).count()
    }

    fn get(&self) -> Vec<ToDo> {
        self.0.clone()
    }

    fn new_todo(&mut self, title: String) {
        self.0.push(ToDo::new(title));
    }
}


#[derive(Debug, Clone, PartialEq)]
struct ToDo {
    title: String,
    completed: bool
}

impl ToDo {
    fn toggle(&mut self) {
        self.completed = !self.completed;
    }

    fn new(title: String) -> ToDo {
        ToDo { title: title, completed: false }
    }
}

// tests can be defined here for all the code outside of the component function
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_todo() {
        let new_todo = ToDo::new("Task".to_string());
        
        assert_eq!(new_todo.title, "Task");
        assert_eq!(new_todo.completed, false);
    }

    #[test]
    fn toggle_todo() {
        let mut new_todo = ToDo::new("Task 2".to_string());
        new_todo.toggle();
        
        assert_eq!(new_todo.title, "Task 2");
        assert_eq!(new_todo.completed, true);
    }

    #[test]
    fn count_todos_remaining() {
        let todos = ToDos(Vec::new());
        assert_eq!(todos.num_remaining(), 0);
    }

    #[test]
    fn add_todo() {
        let mut todos = ToDos(Vec::new());
        todos.new_todo("Task 3".to_string());

        assert_eq!(todos.num_remaining(), 1);
    }

    #[test]
    fn get_todos_list() {
        let mut todos = ToDos(Vec::new());
        todos.new_todo("Task 4".to_string());
        todos.new_todo("Task 5".to_string());

        assert_eq!(
            todos.get(), 
            vec![ToDo::new("Task 4".to_string()), ToDo::new("Task 5".to_string())]
        );
    }

}