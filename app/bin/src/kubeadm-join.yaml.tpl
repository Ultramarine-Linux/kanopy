apiVersion: kubeadm.k8s.io/v1beta3
discovery:
  bootstrapToken:
    apiServerEndpoint: kube-apiserver:6443
    token: {{ cluster.join.token }}
    unsafeSkipCAVerification: true
  timeout: 5m0s
  tlsBootstrapToken: {{ cluster.join.token }}
kind: JoinConfiguration
nodeRegistration:
  criSocket: /var/run/crio/crio.sock
  imagePullPolicy: IfNotPresent
{{%  if hostname %}}
  name: {{ hostname }}
{{%  endif %}}
  taints: null