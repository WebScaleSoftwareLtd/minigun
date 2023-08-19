# frozen_string_literal: true

require 'uri'
require 'json'

module Minigun
    class Response
        attr_reader :code, :body, :headers

        def initialize(response)
            @code = response[:status_code]
            @body = response[:body]
            @headers = response[:headers]
        end

        def ok?
            200 <= @code && @code < 300
        end

        def json
            return if @body.nil?
            JSON.parse(@body)
        end
    end

    class GenericRequestBuilder
        def initialize(url, method)
            # Make sure the URL is either a string or a URI object
            @url = url.is_a?(String) ? URI.parse(url) : url
            raise ArgumentError, "Invalid URL" unless @url.is_a?(URI::Generic)

            # Make sure the method is ^[a-zA-Z]+$.
            raise ArgumentError, "Invalid method" unless method.is_a?(String) && method.match(/^[a-zA-Z]+$/)

            # Make it low_level_<method> so we can call it later.
            @method = "low_level_#{method.downcase}".to_sym
        end

        def run
            Response.new(Minigun.send(@method, @url.to_s, @config))
        end

        def header(key, value)
            headers = get_or_make_config[:headers] ||= {}
            headers[key.to_s] = value.to_s
            self
        end

        def json(data)
            get_or_make_config[:body] = data.respond_to?(:to_json) ? data.to_json : data
            headers = get_or_make_config[:headers] ||= {}
            headers["Content-Type"] = "application/json"
            self
        end

        def string(data)
            get_or_make_config[:body] = data.to_s
            self
        end

        def ignore_body
            get_or_make_config[:read_body] = false
            self
        end

        private

        def get_or_make_config
            @config ||= {}
        end
    end

    class GET < GenericRequestBuilder
        def initialize(url)
            super(url, "GET")
        end
    end

    class POST < GenericRequestBuilder
        def initialize(url)
            super(url, "POST")
        end
    end

    class PUT < GenericRequestBuilder
        def initialize(url)
            super(url, "PUT")
        end
    end

    class DELETE < GenericRequestBuilder
        def initialize(url)
            super(url, "DELETE")
        end
    end

    class PATCH < GenericRequestBuilder
        def initialize(url)
            super(url, "PATCH")
        end
    end
end
