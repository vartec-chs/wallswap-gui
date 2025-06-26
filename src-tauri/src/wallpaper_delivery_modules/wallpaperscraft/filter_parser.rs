use crate::core::{
    debug::Debug,
    api_result::*,
    models::{Category, ResolutionFilter},
    HttpFetcher,
};
use log;
use scraper::{Html, Selector};

pub struct FilterParser<'a> {
    fetcher: &'a HttpFetcher,
    document: Option<Html>,
}

impl<'a> FilterParser<'a> {
    /// Создает новый менеджер категорий
    pub fn new(fetcher: &'a HttpFetcher) -> Self {
        Self {
            fetcher,

            document: None,
        }
    }

    async fn parse_resolutions(&self, document: &Html) -> Result<Vec<ResolutionFilter>> {
        let selector =
            Selector::parse("div.content-sidebar.content-sidebar_shift.gui-hidden-mobile.JS-Fix")
                .unwrap();
        let element = document.select(&selector).next().unwrap();

        Debug::save_html_debug(&element.html(), "resolutions_sidebar.html").await?;
        Debug::preview_html(&element.html(), 10);

        let filters_list_selector = Selector::parse("ul.filters__list.JS-Filters").unwrap();
        let filters_list = element.select(&filters_list_selector).next().unwrap();

        let category_selector = Selector::parse("li.filters__toggler").unwrap();
        let category_name_selector = Selector::parse("span.filters__toggler-text").unwrap();
        let resolutions_ul_selector =
            Selector::parse("div.filters__toggler-content ul.filters__list").unwrap();
        let resolution_li_selector = Selector::parse("li.filter").unwrap();
        let resolution_link_selector = Selector::parse("a.filter__link").unwrap();

        Debug::save_html_debug(&filters_list.html(), "resolutions.html").await?;
        Debug::preview_html(&filters_list.html(), 10);

        let mut resolutions = Vec::new();

        log::info!("🔍 Парсинг разрешений...");

        for category_li in filters_list.select(&category_selector) {
            let category_name = category_li
                .select(&category_name_selector)
                .next()
                .unwrap()
                .text()
                .collect::<String>();
            let resolutions_ul = category_li.select(&resolutions_ul_selector).next().unwrap();

            log::info!("🔍 Парсинг разрешений '{}'", category_name);

            for resolution_li in resolutions_ul.select(&resolution_li_selector) {
                if let Some(resolution_a) = resolution_li.select(&resolution_link_selector).next() {
                    let resolution_text = resolution_a.text().collect::<String>();

                    log::info!("  {}", resolution_text);

                    let href = resolution_a.value().attr("href").unwrap_or("").to_string();
                    resolutions.push(ResolutionFilter::new(
                        resolution_text.trim().to_string(),
                        resolution_text.trim().to_string(),
                        href,
                        Some(category_name.clone()),
                    ));
                }
            }
        }
        Ok(resolutions)
    }
}
