pub struct DoublyLinkedList<'a> {
    command: String,
    next: &'a DoublyLinkedList<'a>,
    prev: &'a DoublyLinkedList<'a>
}