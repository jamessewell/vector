/**
 * Copyright (c) 2017-present, Facebook, Inc.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import React from 'react';

import Head from '@docusaurus/Head';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import useBaseUrl from '@docusaurus/useBaseUrl';
import DocPaginator from '@theme/DocPaginator';
import styles from './styles.module.css';

function Headings({headings, isChild}) {
  if (!headings.length) return null;
  return (
    <ul className={isChild ? '' : 'contents'}>
      {headings.map(heading => (
        <li key={heading.id}>
          <a href={`#${heading.id}`} className="contents__link">
            {heading.value}
          </a>
          <Headings isChild headings={heading.children} />
        </li>
      ))}
    </ul>
  );
}

function DocItem(props) {
  const {siteConfig = {}} = useDocusaurusContext();
  const {url: siteUrl} = siteConfig;
  const {metadata, content: DocContent} = props;
  const {
    delivery_guarantee,
    description,
    editUrl,
    event_types: eventTypes,
    image: metaImage,
    input_types: inputTypes,
    issues_url: issuesUrl,
    keywords,
    lastUpdatedAt,
    lastUpdatedBy,
    output_types: outputTypes,
    permalink,
    source_url: sourceUrl,
    status,
    title,
  } = metadata;

  const metaImageUrl = siteUrl + useBaseUrl(metaImage);

  return (
    <div>
      <Head>
        {title && <title>{title}</title>}
        {description && <meta name="description" content={description} />}
        {description && (
          <meta property="og:description" content={description} />
        )}
        {keywords && keywords.length && (
          <meta name="keywords" content={keywords.join(',')} />
        )}
        {metaImage && <meta property="og:image" content={metaImageUrl} />}
        {metaImage && <meta property="twitter:image" content={metaImageUrl} />}
        {metaImage && (
          <meta name="twitter:image:alt" content={`Image for ${title}`} />
        )}
        {permalink && <meta property="og:url" content={siteUrl + permalink} />}
      </Head>
      <div className="padding-vert--lg">
        <div className="container">
          <div className="row">
            <div className="col">
              <div className={styles.docItemContainer}>
                {!metadata.hide_title && (
                  <header>
                    <div className="badges">
                      {eventTypes && eventTypes.includes("log") && <span className="badge badge--primary" title="This component works with log events.">LOG</span>}
                      {eventTypes && eventTypes.includes("metric") && <span className="badge badge--primary" title="This component works with metric events.">METRIC</span>}
                    </div>
                    <h1 className={styles.docTitle}>{metadata.title}</h1>
                  </header>
                )}
                <article>
                  <div className="markdown">
                    <DocContent />
                  </div>
                </article>
                <div className="margin-vert--lg">
                  <DocPaginator metadata={metadata} />
                </div>
              </div>
            </div>
            {DocContent.rightToc && (
              <div className="col col--3">
                <div className={styles.tableOfContents}>
                  {(status || delivery_guarantee) &&
                    <div className="section">
                      <div className="title">Status</div>
                      {status == "beta" ?
                        <div>
                          <Link to="/docs/about/guarantees#beta" className="text--warning" title="This component is in beta and is not recommended for production environments. Click to learn more.">
                            <i className="feather icon-alert-triangle"></i> beta
                          </Link>
                        </div> :
                        <div>
                          <Link to="/docs/about/guarantees#prod-ready" className="text--primary" title="This component has passed reliability standards that make it production ready. Click to learn more.">
                            <i className="feather icon-award"></i> prod-ready
                          </Link>
                        </div>}
                      {delivery_guarantee == "best_effort" ?
                        <div>
                          <Link to="/docs/about/guarantees#best-effort" className="text--warning" title="This component makes a best-effort delivery guarantee, and in rare cases can lose data. Click to learn more.">
                            <i className="feather icon-shield-off"></i> best-effort
                          </Link>
                        </div> :
                        <div>
                          <Link to="/docs/about/guarantees#at-least-once" className="text--primary" title="This component offers an at-least-once delivery guarantee. Click to learn more.">
                            <i className="feather icon-shield"></i> at-least-once
                          </Link>
                        </div>}
                    </div>}
                  {DocContent.rightToc.length > 0 &&
                    <div className="section">
                      <div className="title">Contents</div>
                      <Headings headings={DocContent.rightToc} />
                    </div>
                  }
                  <div className="section">
                    <div className="title">Resources</div>
                    <ul className="contents">
                      {editUrl && (<li><a href={editUrl} className="contents__link" target="_blank"><i className="feather icon-edit-1"></i> Edit this page</a></li>)}
                      {issuesUrl && (<li><a href={issuesUrl} className="contents__link" target="_blank"><i className="feather icon-message-circle"></i> View Issues</a></li>)}
                      {sourceUrl && (<li><a href={sourceUrl} className="contents__link" target="_blank"><i className="feather icon-github"></i> View Source</a></li>)}
                    </ul>
                  </div>
                  {(lastUpdatedAt || lastUpdatedBy) && (
                    <div className="section">
                      Last updated{' '}
                      {lastUpdatedAt && (
                        <>
                          on{' '}
                          <strong>
                            {new Date(
                              lastUpdatedAt * 1000,
                            ).toLocaleDateString()}
                          </strong>
                          {lastUpdatedBy && ' '}
                        </>
                      )}
                      {lastUpdatedBy && (
                        <>
                          by <strong>{lastUpdatedBy}</strong>
                        </>
                      )}
                    </div>
                  )}
                </div>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}

export default DocItem;
