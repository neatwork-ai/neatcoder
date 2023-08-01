pub fn classify_based_on_project(
    who: &str,
    project: &str,
    question: &str,
    options: &str,
) -> String {
    let prompt = format!(
        "The {who} reaches out to you with the following project\n'''{project}'''
Based on the project description above, {question}:\n{options}",
        who = who,
        project = project,
        question = question,
        options = options
    );

    prompt
}

mod tests {
    #[cfg(test)]
    use super::classify_based_on_project;

    #[test]
    fn test_classify_based_on_project() {
        let project = "Project: Advanced Search Feature Implementation
For a website or an application that already hosts a significant amount of content (such as blog posts, products, resources, etc.), enhancing the search functionality can greatly improve the user experience.        

The goal of this project is to implement an advanced search feature that allows users to conduct more specific and sophisticated searches. Here are a few potential sub-features:
- Search Autocomplete: As users begin typing their search, suggest possible queries or results that match the entered text.
- Filtering and Sorting: Allow users to narrow down search results by various criteria like date, category, popularity, or relevance. They should also be able to sort these results based on these criteria.
- Search by Tag or Category: If the content is categorized or tagged, allow users to search within a specific category or by a particular tag.        
- Fuzzy Search: Users should be able to get relevant results even if they make typographical errors in their search query. The search feature should be intelligent enough to understand the user's intent and return relevant results.        
- Saved Searches: Allow users to save their search criteria or individual searches so that they can easily repeat these searches later.";

        let options = "- Restful API
- RPC API
- Programming Library
- WebHooks
- WebSockets
- Command-Line Interface
- Other (if none of the above fit)";

        let prompt = classify_based_on_project(
            "product_manager",
            project,
            "the interface of the project should be",
            options,
        );

        println!("{}", prompt);
    }
}
