error_chain! {
  errors {
    AlreadyCrawled {
      description("URL is already crawled")
      display("URL is already crawled")
    }

    // TODO: Store reason on why it had failed in the first place.
    PreviouslyFailed {
      description("Crawler had previously failed to retrieve this page")
      display("Crawler had previously failed to retrieve this page")
    }

    // TODO: Implement Relative URL Resolver
    IsRelative {
      description("Relative URL is unimplemented")
      display("Relative URL is unimplemented")
    }

    NonHTTP {
      description("Non-HTTP protocols are unsupported")
      display("Non-HTTP protocols are unsupported")
    }

    // TODO: Add flags to allow scraping clearnet URLs
    IsClearnet {
      description("Clearnet URLs are ignored")
      display("Clearnet URLs are ignored")
    }
  }
}
