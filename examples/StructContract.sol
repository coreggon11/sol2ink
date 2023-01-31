pragma solidity ^0.8.0;

contract StructContract {
    enum Status {
        Pending,
        Shipped,
        Accepted,
        Rejected,
        Canceled
    }

    Status public status;

    event Log(address indexed sender, string message, uint8 priority, Status status);
    event AnotherLog();

    struct Todo {
        string text;
        bool completed;
        uint8 priority;
        string comment;
    }

    Todo[] public todos;

    function get() public view returns (Status) {
        return status;
    }

    function set(Status _status) public {
        status = _status;
    }

    function cancel() public {
        status = Status.Canceled;
    }

    function reset() public {
        delete status;
    }

    function create_events() public {
        emit Log(msg.sender, "log event", 9, Accepted);
        emit AnotherLog();
    }

    function create_todo(string calldata _text, uint8 _priority, string calldata _comment) public {
        // 3 ways to initialize a struct
        // - calling it like a function
        todos.push(Todo(_text, false, _priority, _comment));

        // key value mapping
        todos.push(Todo({text: _text, completed: false, priority: _priority, _comment: _comment}));

        // initialize an empty struct and then update it
        Todo memory todo;
        todo.text = _text;
        // completed initialized to false
        todo.priority = _priority;
        todo.comment = _comment;

        todos.push(todo);
    }
}
