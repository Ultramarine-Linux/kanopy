apiVersion: kubeadm.k8s.io/v1beta3
kind: ClusterConfiguration
kubernetesVersion: v1.26.4
controllerManager:
  extraArgs:
    flex-volume-plugin-dir: "/etc/kubernetes/kubelet-plugins/volume/exec"
networking:
{% if networking.pod-cidr %}
  podSubnet: {{ networking.pod-cidr }}
{% endif %}
{% if networking.service-cidr %}
  serviceSubnet: {{ networking.service-cidr }}
{% endif %}
featureGates:
    NodeSwap: true

---
apiVersion: kubeadm.k8s.io/v1beta3
kind: InitConfiguration
nodeRegistration:
{% if hostname %}
  name: {{ hostname }}
{% endif %}
  criSocket: /var/run/crio/crio.sock
  taints: []
  kubeletExtraArgs:
    cgroup-driver: systemd
    feature-gates: "NodeSwap=true"
