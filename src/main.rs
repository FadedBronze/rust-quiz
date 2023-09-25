use indextree::{Arena, NodeId};

#[derive(Debug)]
struct AnswerAndFollowUp {
    answer: Option<String>,
    question: Option<String>,
}

impl AnswerAndFollowUp {
    fn new(answer: &str) -> Self {
        Self {
            answer: Some(answer.to_string()),
            question: None,
        }
    }
}

struct QuestionBuilder<'a> {
    arena: &'a mut Arena<AnswerAndFollowUp>,
    nodes: Vec<NodeId>,
    idx: usize,
    node: NodeId,
}

impl QuestionBuilder<'_> {
    fn reply<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(Reply),
    {
        f(Reply {
            arena: self.arena,
            answer_node: self.nodes[self.idx],
        });
        self.idx += 1;
        self
    }

    fn end(&mut self, message: &str) -> &mut Self {
        create_question(self.nodes[self.idx], self.arena, message, vec![]);
        self.idx += 1;
        self
    }

    fn play(&mut self) {
        recurse_game(self.arena, self.node);
    }

    fn new<'a>(
        question: &str,
        answers: Vec<&str>,
        arena: &'a mut Arena<AnswerAndFollowUp>,
    ) -> QuestionBuilder<'a> {
        let node = arena.new_node(AnswerAndFollowUp::new(""));

        create_question(node, arena, question, answers)
    }
}

fn create_question<'a>(
    node: NodeId,
    arena: &'a mut Arena<AnswerAndFollowUp>,
    question: &str,
    answers: Vec<&str>,
) -> QuestionBuilder<'a> {
    arena[node].get_mut().question = Some(question.to_owned());

    let mut nodes = vec![];

    for answer in answers.iter() {
        nodes.push(node.append_value(AnswerAndFollowUp::new(answer), arena));
    }

    QuestionBuilder {
        arena,
        nodes,
        node,
        idx: 0,
    }
}

struct Reply<'a> {
    arena: &'a mut Arena<AnswerAndFollowUp>,
    answer_node: NodeId,
}

impl Reply<'_> {
    fn followup(&mut self, question: &str, answers: Vec<&str>) -> QuestionBuilder<'_> {
        create_question(self.answer_node, self.arena, question, answers)
    }
}

fn recurse_game(arena: &mut Arena<AnswerAndFollowUp>, node: NodeId) {
    let letters = "abcdefg";
    let mut nodes = vec![];

    let question = arena[node].get().question.clone();

    if question == None {
        return;
    }

    println!("{}", question.unwrap());

    if node.children(arena).count() == 0 {
        return;
    }

    for (idx, node) in node.children(arena).enumerate() {
        let answer_and_follow_up = arena[node].get();
        let answer = answer_and_follow_up.answer.clone().unwrap();

        nodes.push(node.clone());
        println!("{}: {}", letters.chars().nth(idx).unwrap(), answer);
    }

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line = line.trim().to_owned();

    let idx = letters.find(|char| char == line.chars().nth(0).unwrap());

    recurse_game(arena, nodes[idx.unwrap()]);
}

fn main() {
    // Create a new Arena to hold questions and answers
    let mut arena = Arena::new();

    // Start building the quiz
    QuestionBuilder::new(
      "Welcome to the Geography Quiz!",
      vec!["Start"],
      &mut arena,
  )
  .reply(|mut reply| {
      reply
          .followup("Question 1: What is the capital of France?", vec!["Paris", "Berlin", "London"])
          .reply(|mut reply| {
              reply.followup("Question 2: Which river is the longest in the world?", vec!["Nile", "Amazon", "Mississippi"])
              .reply(|mut reply| {
                  reply.followup("Question 3: Which country is known as the Land of the Rising Sun?", vec!["Japan", "China", "South Korea"])
                  .end("Congratulations! You've completed the Geography Quiz!").end("Incorrect!").end("Incorrect!");
              }).end("Incorrect!").end("Incorrect!");
          }).end("Incorrect!").end("Incorrect!");
  })
  .play();
}
