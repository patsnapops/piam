import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  Svg: React.ComponentType<React.ComponentProps<'svg'>>;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: '多云',
    Svg: require('@site/static/img/multi-cloud.svg').default,
    description: (
      <>
        AWS、腾讯云、阿里云 ...
      </>
    ),
  },
  {
    title: '多帐号',
    Svg: require('@site/static/img/multi-account.svg').default,
    description: (
      <>
        7478、3799、0066 ...
      </>
    ),
  },
  {
    title: '多协议',
    Svg: require('@site/static/img/multi-protocol.svg').default,
    description: (
      <>
        S3、DynamoDB、SQS ...
      </>
    ),
  },
];

function Feature({title, Svg, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
