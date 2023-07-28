// use crate::{
//     ai::openai::{
//         client::OpenAI,
//         job::OpenAIJob,
//         msg::{GptRole, OpenAIMsg},
//     },
//     output::tasks::Tasks,
//     workflows::generative_tree::distribute_tasks,
// };
// use anyhow::Result;

// pub async fn generate_knowledge_graph(client: &OpenAI, job: &OpenAIJob) -> Result<()> {
//     let sys_msg = OpenAIMsg {
//         role: GptRole::System,
//         content: String::from("Given a textual document, produce me a Knowledge Graph representation of its semantical relevant concepts."),
//     };

//     let context = r#"
//         The Knowledge Graph should be represented in JSON format with
//         concepts being represented by 'entities' and 'relations' between these.\n
//     "#;

//     let task = r#"
//         Following the above rules, structure the following text into a Knowledge Graph:

//         ---

//         During the height of the Cold War, when export controls to the Soviet bloc were at their strictest, B.I.S. was a critical hub in the Western defenses, processing up to 100,000 export licenses annually. During the relative peace and stability of the 1990s, the bureau lost some of its raison d’être — as well as staff and funding — and licenses shriveled to roughly 10,000 per year. Today, the number is 40,000 and climbing. With a sprawling trade blacklist known as the entity list (currently 662 pages and counting), numerous pre-existing multilateral export-control agreements and ongoing actions against Russia and China, B.I.S. is busier than ever. “We spend 100 percent of our time on Russia sanctions, another 100 percent on China and the other 100 percent on everything else,” says Matt Borman, the deputy assistant secretary of commerce for export administration.

//         In recent years, semiconductor chips have become central to the bureau’s work. Chips are the lifeblood of the modern economy, and the brains of every electronic device and system, from iPhones to toasters, data centers to credit cards. A new car might have more than a thousand chips, each one managing a different facet of the vehicle’s operation. Semiconductors are also the driving force behind the innovations poised to revolutionize life over the next century, like quantum computing and artificial intelligence. OpenAI’s ChatGPT, for example, was reportedly trained on 10,000 of the most advanced chips currently available.

//         With the Oct. 7 export controls, the United States government announced its intent to cripple China’s ability to produce, or even purchase, the highest-end chips. The logic of the measure was straightforward: Advanced chips, and the supercomputers and A.I. systems they power, enable the production of new weapons and surveillance apparatuses. In their reach and meaning, however, the measures could hardly have been more sweeping, taking aim at a target far broader than the Chinese security state. “The key here is to understand that the U.S. wanted to impact China’s A.I. industry,” says Gregory C. Allen, director of the Wadhwani Center for A.I. and Advanced Technologies at the Center for Strategic and International Studies in Washington. “The semiconductor stuff is the means to that end.”

//         Though delivered in the unassuming form of updated export rules, the Oct. 7 controls essentially seek to eradicate, root and branch, China’s entire ecosystem of advanced technology. “The new policy embodied in Oct. 7 is: Not only are we not going to allow China to progress any further technologically, we are going to actively reverse their current state of the art,” Allen says. C.J. Muse, a senior semiconductor analyst at Evercore ISI, put it this way: “If you’d told me about these rules five years ago, I would’ve told you that’s an act of war — we’d have to be at war.”

//         If the controls are successful, they could handicap China for a generation; if they fail, they may backfire spectacularly, hastening the very future the United States is trying desperately to avoid. The outcome will likely shape U.S.-China competition, and the future of the global order, for decades to come. “There are two dates that will echo in history from 2022,” Allen says. “The first is Feb. 24, when Russia invaded Ukraine; and the second is Oct. 7.”

//         Despite the immense intricacy of their design, semiconductors are, in a sense, quite simple: tiny pieces of silicon carved with arrays of circuits. The circuits flip on and off based on the activity of switches called transistors. When a circuit is on, it produces a one; off, a zero. The first chips, invented in the late 1950s, held only a handful of transistors. Today the primary semiconductor in a new smartphone has between 10 and 20 billion transistors, each about the size of a virus, carved like a layer cake into the structure of the silicon.

//         The rate of progress over the last six decades has been famously described by Moore’s Law, which observed that the number of transistors that can be fit on a chip has roughly doubled every two years. Chris Miller, author of the book “Chip War” and an associate professor of international history at the Fletcher School at Tufts University, likes to note that if airplanes had improved at the same rate as chips, they’d now be flying at several times the speed of light. No technology in the history of human civilization has ever matched the breathtaking ascent of computing power.

//         Editors’ Picks

//         My Friends Excluded Me From Their Awesome Trip. What Should I Do?

//         She Steals Surfboards by the Seashore. She’s a Sea Otter.

//         Men Are Baring Midriffs in Crop Tops
//         Semiconductor-manufacturing plants, known as fabs, are the most expensive factories in the world, conducting the most complex manufacturing ever accomplished, at a scale of production never before achieved with any other device. The wider chip industry, meanwhile, is a web of mutual interdependence, spread all over the planet in highly specialized regions and companies, its feats made possible by supply chains of exceptional length and complexity — a poster child, in other words, for globalization. “It’s hard to imagine how the capabilities they’ve reached would be possible without access to the smartest minds in the world all working together,” Miller says. And yet it is this same interconnectedness that makes the industry vulnerable to regulations like those the Biden administration is pursuing.

//         \n
//     "#;

//     // let output_schema = r#"Format the output in csv format with two columns:
//     // - Item (i.e. Item to work on)
//     // - Role (i.e. Person/Role to work on it)"#;

//     let msg = context.to_string() + task;

//     let user_msg = OpenAIMsg {
//         role: GptRole::User,
//         content: msg,
//     };

//     let resp = client.chat(job, &[&sys_msg, &user_msg], &[], &[]).await?;
//     let json = resp.choices.first().unwrap().message.content.as_str();
//     println!("{}", json);

//     // Implement Fallback logic
//     let tasks: Tasks = serde_json::from_str(json)?;

//     println!("Tasks: {:?}", tasks);

//     distribute_tasks(
//         client,
//         job,
//         &tasks,
//         "
//     Create a Knowledge Graph, in JSON format, from the text. The structure of the Knowledge Graph should follow the previous instructions. \n
//     ",
//     )
//     .await?;

//     Ok(())
// }
